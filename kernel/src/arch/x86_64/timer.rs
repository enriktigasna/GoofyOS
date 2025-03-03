use x86_64::instructions::port::Port;
use x86_64::instructions::interrupts;


pub fn init_pit(frequency: u32) {
    let divisor = 1193182 / frequency;
    interrupts::disable();

    unsafe {
        let mut cmd_port = Port::<u8>::new(0x43);
        let mut data_port = Port::<u8>::new(0x40);

        cmd_port.write(0x36);

        data_port.write((divisor & 0xFF) as u8);
        data_port.write((divisor >> 8) as u8);
    }

    interrupts::enable();
}

