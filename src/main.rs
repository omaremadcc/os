#![no_std]
#![no_main]

use core::panic::{PanicInfo};

#[panic_handler]
unsafe fn panic_handler(_info: &PanicInfo) -> ! {
    return loop {}
}

#[unsafe(no_mangle)]
unsafe fn _start() -> ! {
    return loop {

    }
}
