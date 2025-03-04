use crate::{arch::x86_64::pic::{InterruptIndex, PICS}, print, println};
use x86_64::structures::idt::InterruptStackFrame;

pub(super) extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("Exception: Breakpoint\n{:#?}", stack_frame);
}

pub(super) extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("Exception: DOUBLE FAULT\n{:#?}", stack_frame);
}

pub(super) extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe{ PICS.lock().notify_end_of_interrupt(InterruptIndex::TIMER.as_u8()) };
}
