use lazy_static::lazy_static;
use x86_64::{
    registers::control::Cr3, structures::paging::{page_table::FrameError, PageTable}, PhysAddr, VirtAddr
};
use limine::request::HhdmRequest;

#[used]
#[link_section = ".requests"]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();


lazy_static!{
    pub static ref MAPPER: Mapper = Mapper::new();
}

// TODO: Add level4 page here
pub struct Mapper {
    pub phys_offset: VirtAddr,
}

impl Mapper {
    pub fn new() -> Self {
        let phys_offset = HHDM_REQUEST.get_response().expect("Failed to get hhdm").offset();

        Mapper {
            phys_offset: VirtAddr::new(phys_offset),
        }
    }

    pub fn virt_to_phys(&self, virt: VirtAddr) -> Option<PhysAddr> {
        let (mut frame, _) = Cr3::read();

        let table_indeces = [
          virt.p4_index(), virt.p3_index(), virt.p2_index(), virt.p1_index()
        ];

        for &index in &table_indeces {
            let table_ptr: *const PageTable = virt.as_ptr();
            let table = unsafe {&*table_ptr};

            let entry = &table[index];
            frame = match entry.frame() {
                Ok(frame) => frame,
                Err(FrameError::FrameNotPresent) => return None,
                Err(FrameError::HugeFrame) => panic!("Don't support huge frames yet")
            };
        }

        Some(frame.start_address() + u64::from(virt.page_offset()))
    }

    // Get DMA virtual page from physical address
    pub fn phys_to_virt(&self, phys: PhysAddr) -> VirtAddr {
        VirtAddr::new(phys.as_u64() + self.phys_offset.as_u64())
    }
}
