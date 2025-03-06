// Each slab has a descriptor that is in the beginning
// 2048 SLABS Are special: 512 + 1024 + 2048
// 1024 SLABS Are also special: 512 + 1024*3
// 512 slabs and smaller are all only filled with chunks of their size

use spin::Mutex;

use crate::{mm::page_alloc::PAGEALLOC, println};

pub static mut SLABALLOC: Mutex<Option<SlabAlloc>> = Mutex::new(None);


pub const SLAB_SIZES: [u32; 8] = [16, 32, 64, 128, 256, 512, 1024, 2048];


// Like the page allocator, when allocating check freelist and do following
//  if size == 1: dereference SlabFreelist and go to next
//  if size > 1: push SlabFreelist structure to next slab entry (Size forward) and pop one
//  if none left, allocate new cache and add to SlabDescriptor

pub struct SlabAlloc {
    freelist_heads: [Option<*mut SlabFreelist>; 8]
}

impl SlabAlloc {
    pub fn new() -> Self {
        // Preallocate a page for each type of slab and initialize the freelist
        let mut slab_alloc = SlabAlloc {
            freelist_heads: [None; 8]
        };

        for size in SLAB_SIZES {
            unsafe { Self::create_slab(&mut slab_alloc, size); }
        }

        slab_alloc
    }

    unsafe fn create_slab(&mut self, size: u32) -> *mut SlabDescriptor {
        // Dead slab space should be filled with 512s at least
        // Because otherwise there will be wasted space
        /* ┌──────────────┐◄──────────── 16B   SlabDescriptor
         * ├──────────────┤◄──────────── 504B  Unused space
         * ├──────────────┤◄──────────── 512B  Slab
         * ├──────────────┤◄──────────── 1024B Slab
         * │              │
         * ├──────────────┤◄──────────── 2048B Slab
         * │              │
         * │              │
         * │              │
         * └──────────────┘
         */

        let page_ptr: *mut SlabDescriptor = unsafe {
            PAGEALLOC.lock()
                .as_mut()
                .expect("Page Allocator Uninitialized")
                .alloc_page() as *mut SlabDescriptor
        };

        core::ptr::write(page_ptr, SlabDescriptor {
            size: size as u16
        });

        match size {
            // If size > 512, Add smaller chunks in the page to fill out
            1024 => {
                let freelist_ptr  = page_ptr.byte_add(512) as *mut SlabFreelist;
                self.write_freelist(freelist_ptr, 1, 512);
            },
            2048 => {
                let freelist_ptr  = page_ptr.byte_add(512) as *mut SlabFreelist;
                self.write_freelist(freelist_ptr, 1, 512);
                
                let freelist_ptr  = page_ptr.byte_add(1024) as *mut SlabFreelist;
                self.write_freelist(freelist_ptr, 1, 1024);
            },
            _ => ()
        }

        let freelist_ptr  = page_ptr.byte_add(size as usize) as *mut SlabFreelist;
        self.write_freelist(freelist_ptr, 4096/(size as u32) - 1 as u32, size);


        page_ptr
    }

    unsafe fn write_freelist(&mut self, freelist_ptr: *mut SlabFreelist, nfree: u32, size: u32) {
        let freelist_index = SLAB_SIZES.iter().position(|n| *n == size).expect("Trying to create an invalid size");

        core::ptr::write(freelist_ptr, SlabFreelist {
            next: self.freelist_heads[freelist_index].unwrap_or(core::ptr::null_mut()),
            nfree
        });

        self.freelist_heads[freelist_index] = Some(freelist_ptr);
    }
 
    pub fn alloc_chunk(&mut self, size: u32) -> *mut u8 {
        // Make size into correct size
        if size > 2048 {
            panic!("Slab allocator trying to alloc {size} max size is 2048");
        }

        let chunk_size = Self::chunk_size(size);

        let freelist_index = SLAB_SIZES.iter().position(|n| *n == chunk_size).unwrap();

        if self.freelist_heads[freelist_index].is_none() {
            unsafe { self.create_slab(chunk_size); }
        }

        // nfree = 1 => Dereference freelist_heads
        // nfree > 1 => Copy slab-freelist forward one chunk and nfree -= 1
        let freelist_ptr: *mut SlabFreelist = self.freelist_heads[freelist_index].unwrap();

        unsafe {
            match (*freelist_ptr).nfree {
                0 => panic!("Corrupted slab allocator"),
                1 => {
                    if (*freelist_ptr).next.is_null() {
                        self.freelist_heads[freelist_index] = None
                    } else {
                        self.freelist_heads[freelist_index] = Some((*freelist_ptr).next)
                    }
                }
                _ => {
                    let new_head = freelist_ptr.byte_add(chunk_size as usize);
                    core::ptr::write(new_head, SlabFreelist {
                        next: (*freelist_ptr).next,
                        nfree: (*freelist_ptr).nfree-1
                    });

                    self.freelist_heads[freelist_index] = Some(new_head)
                }
            }
        }

        freelist_ptr as *mut u8
    }

    pub unsafe fn free_chunk(&mut self, chunk: *mut u8) {
        // Get slab size of it
        let chunk_size = Self::get_chunk_size(chunk);
        let freelist_index = SLAB_SIZES.iter().position(|n| *n == chunk_size).expect("Slab allocator corrupted, invalid chunk size");
        let freelist_ptr = chunk as *mut SlabFreelist;

        core::ptr::write(freelist_ptr, SlabFreelist {
            next: self.freelist_heads[freelist_index].unwrap_or(core::ptr::null_mut()),
            nfree: 1
        });

        self.freelist_heads[freelist_index] = Some(freelist_ptr);
    }

    fn chunk_size(n: u32) -> u32 {
        if n < 16 {
            return 16
        }

        if n & (n-1) == 0 {
            return n
        }

        return 1 << 32 - n.leading_zeros()
    }

    // And the pointer to get page base
    // Then do some shit to get correct
    unsafe fn get_chunk_size(chunk: *mut u8) -> u32 {
        let slab_descriptor = (chunk as u64 & !0xfff) as *mut SlabDescriptor;
        let slab_size = (*slab_descriptor).size;

        match slab_size {
            1024 => {
                let page_offset = chunk as u64 & 0xfff;
                if page_offset >= 1024 {
                    1024
                } else {
                    512
                }
            }
            2048 => {
                let page_offset = chunk as u64 & 0xfff;
                if page_offset >= 2048 {
                    2048
                } else if page_offset >= 1024 {
                    1024
                } else {
                    512
                }
            }
            _ => return slab_size.into()
        }
    }
}

pub struct SlabDescriptor {
    size: u16
}

#[repr(C)]
pub struct SlabFreelist {
    next: *mut SlabFreelist,
    nfree: u32
}

pub fn init_global_slab() {
    // Allocate a page and 
    unsafe {
        if SLABALLOC.lock().is_some() {
            panic!("Already initialized slab allocator");
        }

        SLABALLOC = Mutex::new(Some(SlabAlloc::new()));
    }
}
