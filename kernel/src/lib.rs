#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod vga;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut buffer = vga::buffer::Buffer::default();
    let string = "Hello World";
    let color = vga::color::ColorCode::default();

    match buffer.write_string(string, color){
        Ok(()) => buffer.flush(),
        Err(_) => {
            unsafe {
                let vgabuf = 0xb8000 as *mut u8;
                vgabuf.write_volatile(b'P');
            }
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}