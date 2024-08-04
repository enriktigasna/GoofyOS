use core::arch::asm;

use crate::println;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("PANIC in {} at line {}", location.file(), location.line())
    } else {
        println!("PANIC at unkown location");
    }

    println!("{}", info.message());

    hcf();
}

pub fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
