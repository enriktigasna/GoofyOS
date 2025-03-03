#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use limine::BaseRevision;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    goofy_os::init();
    loop {}
}
