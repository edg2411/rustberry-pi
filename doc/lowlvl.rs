#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

mod boot {
    use core::arch::global_asm;

    global_asm!(
        ".section .text.__start"
    );
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let gpio_fsel2 = 1<<3;

    unsafe {
        core::ptr::write_volatile(0x3F200008 as *mut u32, gpio_fsel2);
    }
    loop {
        unsafe {
            core::ptr::write_volatile(0x3F20001C as *mut u32, 1<<21);
            for _ in 0..50000 {
                asm!("nop")
            }
            core::ptr::write_volatile(0x3F200028 as *mut u32, 1<<21);
            for _ in 0..50000 {
                asm!("nop")
            }
        }
    }
}

#[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}