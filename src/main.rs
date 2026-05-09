#![no_std]
#![no_main]

use core::panic::{PanicInfo};

use omar_os::vga_text::write;

#[panic_handler]
unsafe fn panic_handler(_info: &PanicInfo) -> ! {
    return loop {}
}

#[unsafe(no_mangle)]
unsafe fn _start() -> ! {
    write();
    return loop {

    }
}
