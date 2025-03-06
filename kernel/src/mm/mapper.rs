use lazy_static::lazy_static;
use x86_64::{
    registers::control::Cr3, 
    structures::paging::{page_table::FrameError, PageTable, PageTableFlags, PhysFrame, Size4KiB}, PhysAddr, VirtAddr,
    instructions::tlb
};
use limine::request::HhdmRequest;

use crate::mm::page_alloc::PAGEALLOC;

#[used]
#[link_section = ".requests"]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();


lazy_static!{
    pub static ref MAPPER: Mapper = Mapper::new();
}

pub struct Mapper {
    pub phys_offset: VirtAddr,
}

#[derive(Debug)]
pub enum MappingError {
    AlreadyMapped,
    PageTableNotPresent,
    InvalidFrame
}

impl Mapper {
    pub fn new() -> Self {
        let phys_offset = HHDM_REQUEST.get_response().expect("Failed to get hhdm").offset();

        Mapper {
            phys_offset: VirtAddr::new(phys_offset),
        }
    }

    pub fn map_to(&self, virt: VirtAddr, phys: PhysAddr, flags: PageTableFlags) ->  Result<(), MappingError> {
        let (level_4_frame, _) = Cr3::read();
        let level_4_table_phys = level_4_frame.start_address();
        let level_4_table_virt = self.phys_to_virt(level_4_table_phys);

        let p4_index = virt.p4_index();
        let p3_index = virt.p3_index();
        let p2_index = virt.p2_index();
        let p1_index = virt.p1_index();

        let p4_table = unsafe { &mut *(level_4_table_virt.as_mut_ptr() as *mut PageTable) };
        let p3_table = self.index_table(p4_table, p4_index.into())?;
        let p2_table = self.index_table(p3_table, p3_index.into())?;
        let p1_table = self.index_table(p2_table, p2_index.into())?;

        let frame = PhysFrame::<Size4KiB>::containing_address(phys);
        let entry = &mut p1_table[p1_index];

        if !entry.is_unused() {
            return Err(MappingError::AlreadyMapped);
        }

        entry.set_frame(frame, flags);
        tlb::flush(virt);

        Ok(())
    }

    pub fn index_table(&self, table: &mut PageTable, index: usize) -> Result<&mut PageTable, MappingError> {
        let entry = &mut table[index];

        if entry.is_unused() {
            let page_ptr = unsafe {
                PAGEALLOC.lock()
                    .as_mut()
                    .expect("Page Allocator Uninitialized")
                    .alloc_page()
            };

            unsafe {
                core::ptr::write_bytes(page_ptr, 0, 0x1000)
            }

            let new_table_phys = PhysAddr::new(page_ptr as u64 - self.phys_offset.as_u64());
            let frame = PhysFrame::<Size4KiB>::containing_address(new_table_phys);

            entry.set_frame(frame, PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
        } else if !entry.flags().contains(PageTableFlags::PRESENT) {
            return Err(MappingError::PageTableNotPresent);
        }

        let frame = entry.frame().map_err(|_| MappingError::InvalidFrame)?;
        let table_phys = frame.start_address();
        let table_virt = self.phys_to_virt(table_phys);

        Ok(unsafe { &mut *(table_virt.as_mut_ptr() as *mut PageTable)})
    }

    pub fn virt_to_phys(&self, virt: VirtAddr) -> Option<PhysAddr> {
        let (mut frame, _) = Cr3::read();

        let table_indeces = [
            virt.p4_index(), virt.p3_index(), virt.p2_index(), virt.p1_index()
        ];

        for &index in &table_indeces {
            let virt_table_ptr = self.phys_offset + frame.start_address().as_u64();
            let table_ptr: *const PageTable = virt_table_ptr.as_ptr();
            let table = unsafe {&*table_ptr};

            let entry = &table[index];
            frame = match entry.frame() {
                Ok(frame) => frame,
                Err(FrameError::FrameNotPresent) => return None,
                Err(FrameError::HugeFrame) => {
                    return Some(frame.start_address() + u64::from(virt.page_offset()));
                }
            };
        }

        Some(frame.start_address() + u64::from(virt.page_offset()))
    }

    // Get DMA virtual page from physical address
    pub fn phys_to_virt(&self, phys: PhysAddr) -> VirtAddr {
        VirtAddr::new(phys.as_u64() + self.phys_offset.as_u64())
    }
}
