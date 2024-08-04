use core::ptr::addr_of;

use lazy_static::lazy_static;
use x86_64::{
    instructions::tables::load_tss, registers::segmentation::{Segment, Segment64, CS, DS, ES, FS}, structures::{
        gdt::{ Descriptor, GlobalDescriptorTable, SegmentSelector },
        tss::TaskStateSegment,
    }, VirtAddr
};

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[0] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            // Have to allow unsafe because rust_analyzer is mad for some reason
            #[allow(unused_unsafe)]
            let stack_start = VirtAddr::from_ptr(unsafe { addr_of!(STACK) });
            let stack_end = stack_start + (STACK_SIZE as u64);

            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let kernel_code = gdt.append(Descriptor::kernel_code_segment());
        let kernel_data = gdt.append(Descriptor::kernel_data_segment());
        let user_code = gdt.append(Descriptor::user_code_segment());
        let user_data = gdt.append(Descriptor::user_data_segment());
        let tss = gdt.append(Descriptor::tss_segment(&TSS));
        
        (gdt, Selectors {kernel_code, kernel_data, user_code, user_data, tss})
    };
}

struct Selectors {
    kernel_code: SegmentSelector,
    kernel_data: SegmentSelector,
    user_code: SegmentSelector,
    user_data: SegmentSelector,
    tss: SegmentSelector
}

pub fn init_gdt() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.kernel_code);
        DS::set_reg(GDT.1.kernel_data);
        // ES::set_reg(GDT.1.user_code);
        // FS::set_reg(GDT.1.user_data);
        load_tss(GDT.1.tss);
    }
}
