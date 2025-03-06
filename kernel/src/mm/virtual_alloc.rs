use spin::Mutex;
use x86_64::{structures::paging::PageTableFlags, VirtAddr};

use crate::mm::page_alloc::PAGEALLOC;

use super::mapper::MAPPER;

pub static mut VIRTUALALLOC: Mutex<Option<VirtualAlloc>> = Mutex::new(None);

pub struct VirtualAlloc {
    vma_base: u64,
}

impl VirtualAlloc {
    pub fn new() -> Self {
        VirtualAlloc {
            vma_base: MAPPER.phys_offset.as_u64() + 0x100000000000
        }
    }

    pub unsafe fn reserve_pages(&mut self, npages: u64) -> VirtAddr {
        let alloc = self.vma_base;
        self.vma_base += npages*0x1000;

        VirtAddr::new(alloc)
    }

    // TODO: When allocating, make a disctinction on whether Slab is initialized or not
    // If slab is initialized, then add to sizelist
    // Before slab is initialized, we will not have to free
    pub unsafe fn vmalloc_pages(&mut self, npages: u64) -> *mut u8 {
        let base = self.reserve_pages(npages);
        for page in 0..npages {
            let page_ptr: *mut u8 = PAGEALLOC.lock().as_mut().expect("Page Allocator Uninitialized").alloc_page();
            let virt: VirtAddr = VirtAddr::new(page_ptr as u64);

            let phys = MAPPER.virt_to_phys(virt).unwrap();

            let target = base.as_u64() + page*0x1000;
            MAPPER.map_to(VirtAddr::new(target), phys, PageTableFlags::PRESENT | PageTableFlags::WRITABLE).expect("Failed to map vmalloc page");
        }

        base.as_mut_ptr()
    }
}

#[allow(static_mut_refs)]
pub fn init_global_vmalloc() {
    unsafe {
        if VIRTUALALLOC.lock().is_some() {
            panic!("Already initialized virtual allocator");
        }

        VIRTUALALLOC = Mutex::new(Some(VirtualAlloc::new()));
    }
}
