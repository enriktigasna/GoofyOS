use limine::{memory_map::{Entry, EntryType}, request::MemoryMapRequest};
use spin::Mutex;
use x86_64::structures::paging::Page;
use core::ptr;

use crate::println;

pub static mut PAGEALLOC: Mutex<Option<PageAlloc>> = Mutex::new(None);

#[used]
#[link_section = ".requests"]
static MMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();


#[repr(C)]
pub struct PageFreelist {
    next: Option<*mut PageFreelist>,
    size: u64
}

pub struct PageAlloc {
    freelist_head: Option<*mut PageFreelist>,
    free_pages: u64,
    memory_map: &'static [&'static Entry]
}

impl PageAlloc {
    pub fn new() -> Self {
        let memory_map = MMAP_REQUEST.get_response().expect("Failed to get memory map").entries();

        let mut pagealloc = PageAlloc {
                freelist_head: None,
                free_pages: 0,
                memory_map
            };

        for &entry in memory_map {
            if entry.entry_type == EntryType::USABLE {
                println!("Base {:x}\nLength {:x}\n\n", entry.base, entry.length);

                let node_ptr = entry.base as *mut PageFreelist;
                unsafe {
                    ptr::write(node_ptr, PageFreelist {
                        next: pagealloc.freelist_head,
                        size: entry.length / 0x1000
                    });
                }
                
                pagealloc.freelist_head = Some(node_ptr);
                pagealloc.free_pages += entry.length / 0x1000;
            }
        }

        pagealloc
    }

    pub fn alloc_page(&mut self) -> *mut u8 {
        todo!()
    }
}

#[allow(static_mut_refs)]
pub fn init_global_pagealloc() {
    unsafe {
        if PAGEALLOC.lock().is_some() {
            panic!("Already initialized page allocator");
        }

        PAGEALLOC = Mutex::new(Some(PageAlloc::new()));
    }
}
