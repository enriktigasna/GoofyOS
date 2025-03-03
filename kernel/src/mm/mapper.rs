use x86_64::{PhysAddr, VirtAddr};
use limine::request::HhdmRequest;

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: HhdmRequest = HhdmRequest::new();


/* Memory Mapper
 * Should only be created once
 * Used to convert physical and virtual adresses
*/

pub struct Mapper {
    phys_offset: VirtAddr
}

impl Mapper {
    pub fn virt_to_phys(&self, virt: VirtAddr) -> Option<PhysAddr> {
        // Implement through first level 4 frame then go through each index and frame
        //
        // Then test through translating in lib
        //
        // let table_indexes = [
        //   addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
        // ];
        todo!()
    }

    pub fn phys_to_virt(&self, phys: PhysAddr) -> Option<VirtAddr> {
        todo!()
    }


}
