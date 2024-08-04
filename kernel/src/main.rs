#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use limine::BaseRevision;

use goofy_os::panic::handler::hcf;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    goofy_os::init();
    hcf();
}
