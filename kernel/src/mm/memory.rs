use x86_64::{
    registers::control::Cr3,
    structures::paging::PageTable,
    VirtAddr
};
use spin::once;

// pub static

pub unsafe fn active_level4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    let (level4, _) = Cr3::read();
    let phys = level4.start_address();
    let virt = physical_memory_offset + phys.as_u64();

    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}
