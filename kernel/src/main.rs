#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use font::FONT;
use gdt::init_gdt;
use interrupts::init_idt;
use limine::BaseRevision;

mod font;
mod framebuffer;
mod gdt;
mod interrupts;
mod panic;
mod terminal;

use framebuffer::{Color, Framebuffer};
use panic::hcf;
use terminal::Terminal;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    let framebuffer = Framebuffer::new().unwrap();
    let terminal = Terminal::new(framebuffer, Color(0, 0, 0), Color(255, 255, 255), FONT);
    terminal.init_global();

    println!(
        "
 d888b   .d88b.   .d88b.  d88888b db    db       .d88b.  .d8888.
88' Y8b .8P  Y8. .8P  Y8. 88'     `8b  d8'      .8P  Y8. 88'  YP
88      88    88 88    88 88ooo    `8bd8'       88    88 `8bo.
88  ooo 88    88 88    88 88~~~      88         88    88   `Y8b.
88. ~8~ `8b  d8' `8b  d8' 88         88         `8b  d8' db   8D
 Y888P   `Y88P'   `Y88P'  YP         YP          `Y88P'  `8888Y'
"
    );
    println!("Welcome to GoofyOS!");
    init_gdt();
    init_idt();

    hcf();
}
