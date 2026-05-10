#![no_std]
#![no_main]

use core::panic::{PanicInfo};
use omar_os::println;


#[panic_handler]
unsafe fn panic_handler(_info: &PanicInfo) -> ! {
    return loop {}
}

#[unsafe(no_mangle)]
unsafe fn _start() -> ! {
    println!("Omar {}", 3);
    return loop {

    }
}
