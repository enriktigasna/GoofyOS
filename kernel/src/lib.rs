#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod arch;
pub mod panic;
mod drivers;
mod tty;
mod mm;

use drivers::framebuffer::{Color, Framebuffer};
use arch::x86_64::{gdt::init_gdt, timer::init_pit};
use arch::x86_64::idt::init_idt;
use arch::x86_64::pic::PICS;
use mm::page_alloc::{init_global_pagealloc, PAGEALLOC};
use mm::slab_alloc::{init_global_slab, SlabFreelist, SLABALLOC};
use mm::virtual_alloc::{init_global_vmalloc, VIRTUALALLOC};
use tty::terminal::Terminal;


pub fn init() {
    let framebuffer = Framebuffer::new().expect("Failed to get framebuffer");
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
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    init_pit(100);
    init_global_pagealloc();
    init_global_vmalloc();
    init_global_slab();

    unsafe {
        // Allocate 16 pages and print
        for i in 0..8 {
            let ptr = VIRTUALALLOC.lock().as_mut().unwrap().vmalloc_pages(0x1000);
            println!("16 MiB continous virtual alloc at {:x}", ptr as usize)
        }
    }

    unsafe {
        for _ in 0..0x1000 {
            let _ptr = SLABALLOC.lock().as_mut().unwrap().alloc_chunk(1024);
            let _ptr = SLABALLOC.lock().as_mut().unwrap().alloc_chunk(16);
        }

        for _ in 0..2 {
            for sz in [16, 32, 64, 128, 256, 512, 1024, 2048] {
                let ptr = SLABALLOC.lock().as_mut().unwrap().alloc_chunk(sz);
                println!("Allocated chunk of size {} at 0x{:x}", sz, ptr as usize);
                ptr.write_bytes(0x41, sz as usize);
                SLABALLOC.lock().as_mut().unwrap().free_chunk(ptr);
                println!("Freed chunk of size {} at 0x{:x}", sz, ptr as usize);
            }
            println!("--------------------------------------");
        }
    }
}
