#[repr(packed)]
pub struct InterruptsDescriptorTable {
    pub entries: [IdtEntry; 256],
}

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct IdtEntry {
    pub pointer_low: u16,
    pub segment_selector: u16,
    pub options: u16,
    pub pointer_middle: u16,
    pub pointer_upper: u32,
    pub reserved: u32,
}
impl IdtEntry {
    pub fn set_handler(&mut self, handler: u64) {
        self.pointer_low = handler as u16;
        self.pointer_middle = (handler >> 16) as u16;
        self.pointer_upper = (handler >> 32) as u32;
        self.segment_selector = 0x8;
        self.options = 0x8E00;
        self.reserved = 0;
    }

    pub const fn missing() -> Self {
        Self {
            pointer_low: 0,
            pointer_middle: 0,
            pointer_upper: 0,
            segment_selector: 0,
            options: 0,
            reserved: 0,
        }
    }
}

#[allow(dead_code)]
#[repr(packed)]
struct Idtr {
    limit: u16,
    base: u64,
}

use core::arch::{asm, naked_asm};

pub unsafe fn load_idt() {
    unsafe {
        let idt_ptr = core::ptr::addr_of_mut!(IDT);
        IDT.entries[3].set_handler(breakpoint_handler_stub as u64);
        IDT.entries[8].set_handler(double_fault_handler_stub as u64);
        IDT.entries[32].set_handler(timer_handler_stub as u64);

        let ptr = Idtr {
            limit: (core::mem::size_of::<InterruptsDescriptorTable>() - 1) as u16,
            base: idt_ptr as *const _ as u64,
        };

        asm!("lidt [{}]", in(reg) &ptr, options(readonly, nostack, preserves_flags));
    }
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
unsafe extern "C" fn common_interrupt_handler() {
    naked_asm!(
        // 1. SAVE ALL REGISTERS
        "push r15",
        "push r14",
        "push r13",
        "push r12",
        "push rbp",
        "push rbx",
        "push r11",
        "push r10",
        "push r9",
        "push r8",
        "push rdi",
        "push rsi",
        "push rdx",
        "push rcx",
        "push rax",
        // 2. CALL RUST
        // Pass the stack pointer (RSP) as the first argument (RDI)
        "mov rdi, rsp",
        "call rust_handler",
        // 3. RESTORE ALL REGISTERS
        "pop rax",
        "pop rcx",
        "pop rdx",
        "pop rsi",
        "pop rdi",
        "pop r8",
        "pop r9",
        "pop r10",
        "pop r11",
        "pop rbx",
        "pop rbp",
        "pop r12",
        "pop r13",
        "pop r14",
        "pop r15",
        // 4. CLEANUP & RETURN
        "add rsp, 16", // Remove vector number and error code
        "iretq",
    );
}

#[repr(C)]
#[derive(Debug)]
pub struct SavedRegisters {

    // Pushed by common_stub (in reverse order of push)
    rax: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    rbx: u64,
    rbp: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,

    // Pushed by individual stub
    vector_number: u64,
    error_code: u64,

    // Pushed by CPU hardware automatically
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_handler(regs: &SavedRegisters) {
    // You now have full access to the CPU state!
    println!("Interrupt {} occurred!", regs.vector_number);
    println!("RIP was at: {:#x}", regs.instruction_pointer);

    println!("vector number: {}", regs.vector_number);

    // If it's a breakpoint, we can just return.
    // If it's a timer, we'd signal the EOI (End of Interrupt).
}

static mut IDT: InterruptsDescriptorTable = InterruptsDescriptorTable {
    entries: [IdtEntry::missing(); 256],
};


#[unsafe(naked)]
pub extern "C" fn breakpoint_handler_stub() {
    naked_asm!(
        "push 0", // Dummy error code
        "push 3", // Interrupt vector 3 (Breakpoint)
        "jmp common_interrupt_handler",
    );
}
#[unsafe(naked)]
pub extern "C" fn double_fault_handler_stub() {
    naked_asm!(
        "push 0", // Dummy error code
        "push 8", // Interrupt vector 6 (Double Fault)
        "jmp common_interrupt_handler",
    );
}

#[unsafe(naked)]
pub extern "C" fn timer_handler_stub() {
    naked_asm!(
        "push 0",
        "push 32",
        "jmp common_interrupt_handler",
    );
}
