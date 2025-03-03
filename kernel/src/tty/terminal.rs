use core::fmt::{self, Write};

use crate::drivers::framebuffer::{Color, Framebuffer};
use super::font::FONT;

pub static mut TERMINAL: Option<Terminal> = None;

pub struct Terminal {
    pub background: Color,
    pub foreground: Color,
    pub framebuffer: Framebuffer,
    pub font: [[u8; 16]; 256],
    pub rows: usize,
    pub cols: usize,
    pub cursor: (usize, usize),
}

impl Terminal {
    pub fn new(
        framebuffer: Framebuffer,
        background: Color,
        foreground: Color,
    ) -> Self {
        let rows = framebuffer.height / 16;
        let cols = framebuffer.width / 8;

        Terminal {
            background,
            foreground,
            framebuffer,
            font: FONT,
            rows,
            cols,
            cursor: (0, 0),
        }
    }

    pub fn init_global(self) {
        unsafe {
            TERMINAL = Some(self);
        }
    }

    // TODO: Make the fucking putchar
    // With match and shit

    pub fn write_char(&self, x: usize, y: usize, char: u8) {
        let x = x * 8;
        let y = y * 16;
        let char = &self.font[char as usize];
        self.framebuffer
            .draw_bitmap_8x16(x, y, &self.foreground, &self.background, char);
    }
}

impl core::fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        for byte in bytes {
            let x = self.cursor.0 * 8;
            let y = self.cursor.1 * 16;

            match byte {
                b'\n' => {
                    self.cursor.0 = 0;
                    self.cursor.1 += 1;
                }
                ascii_char => {
                    self.framebuffer.draw_bitmap_8x16(
                        x,
                        y,
                        &self.foreground,
                        &self.background,
                        &self.font[*ascii_char as usize],
                    );

                    if self.cursor.0 < self.cols - 2 {
                        self.cursor.0 += 1;
                    } else {
                        self.cursor.0 = 0;
                        self.cursor.1 += 1;
                    }
                }
            }
        }

        Ok(())
    }
}

// Override macros

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::tty::terminal::_print(format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    unsafe {
        if let Some(terminal) = TERMINAL.as_mut() {
            terminal.write_fmt(args).expect("Failed to print");
        }
    }
}
