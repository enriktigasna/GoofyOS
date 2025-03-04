use limine::{memory_map::{Entry, EntryType}, request::MemoryMapRequest};
use spin::Mutex;
use x86_64::{PhysAddr, VirtAddr};
use core::ptr;

use crate::{mm::mapper::MAPPER, println};

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
                let node_ptr: *mut PageFreelist = MAPPER.phys_to_virt(PhysAddr::new(entry.base)).as_mut_ptr();
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

    pub unsafe fn alloc_page(&mut self) -> *mut u8 {
        // If size > 1, push freelist head to next page
        // If size == 1, set head to next
        
        let head = self.freelist_head.expect("Out of memory");
        match (*head).size {
            0 => panic!("Corrupted page allocator state"),
            1 => {
                self.freelist_head = (*head).next
            },
            _ => {
                let new_head: *mut PageFreelist = head.byte_add(0x1000);
                ptr::write(new_head, PageFreelist {
                    next: (*head).next,
                    size: (*head).size-1
                });

                self.freelist_head = Some(new_head)
            }
        }

        return head as *mut u8
    }

    pub unsafe fn dealloc_page(&mut self, ptr: *mut u8) {
        // write a PageFreelist in it and set it to head
        let ptr = ptr as *mut PageFreelist;
        ptr::write(ptr, PageFreelist {
            next: self.freelist_head,
            size: 1
        });

        self.freelist_head = Some(ptr);
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
