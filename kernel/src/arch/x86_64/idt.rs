use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use super::interrupts::breakpoint_handler;
use super::interrupts::double_fault_handler;
use super::interrupts::timer_interrupt_handler;
use super::pic::InterruptIndex;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(0);
        }

        idt[InterruptIndex::TIMER.as_u8()].set_handler_fn(timer_interrupt_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

