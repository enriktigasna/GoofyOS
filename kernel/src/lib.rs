#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod arch;
pub mod panic;
mod drivers;
mod tty;

use drivers::framebuffer::{Color, Framebuffer};
use arch::x86_64::{gdt::init_gdt, timer::init_pit};
use arch::x86_64::idt::init_idt;
use arch::x86_64::pic::PICS;
use tty::terminal::Terminal;
use x86_64::instructions::interrupts::int3;


pub fn init() {
    let framebuffer = Framebuffer::new().unwrap();
    let terminal = Terminal::new(framebuffer, Color(0, 0, 0), Color(255, 255, 255));
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
    int3();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    
    init_pit(1);
}
