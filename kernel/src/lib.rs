#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        let vga_buffer = 0xb8000 as *mut u8;

        *vga_buffer = b'H';
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}