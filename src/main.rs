#![no_std]
#![no_main]

use core::panic::{PanicInfo};
use omar_os::{interrupts::load_idt, println};


#[panic_handler]
unsafe fn panic_handler(_info: &PanicInfo) -> ! {
    return loop {}
}

#[unsafe(no_mangle)]
unsafe fn _start() -> ! {
    unsafe {
        load_idt();
    }
    println!("Omar {}", 3);

    println!("Triggering breakpoint...");

        unsafe {
            core::arch::asm!("int3");
        }

    println!("It worked! We returned from the interrupt.");
    return loop {

    }
}
