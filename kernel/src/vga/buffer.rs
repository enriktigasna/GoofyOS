use super::{char::{Char, CharError}, color::ColorCode};

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(C)]
#[allow(dead_code)]
pub struct Buffer {
    chars: [[Char; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[allow(dead_code)]
impl Buffer {
    pub fn write_string(&mut self, s: &str, color: ColorCode) -> Result<(), CharError> {
        let mut i = 0;
        for c in s.chars() {
            if i >= BUFFER_WIDTH * BUFFER_HEIGHT {
                return Err(CharError::BufferTooSmall);
            }
            self.chars[i / BUFFER_WIDTH][i % BUFFER_WIDTH] = Char {
                character: c as u8,
                color,
            };
            i += 1;
        }
        Ok(())
    }

    pub fn flush(&self) {
        unsafe {
            let vgabuf = 0xb8000 as *mut u8;
            for row in 0..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let char = self.chars[row][col];
                    let color = char.color;
                    let offset = (row * BUFFER_WIDTH + col) * 2;
                    *vgabuf.offset(offset as isize) = char.character;
                    *vgabuf.offset(offset as isize + 1) = color.get();
                }
            }
        }
    }

    pub fn default() -> Buffer {
        Buffer {
            chars: [[Char {
                character: b' ',
                color: ColorCode::default(),
            }; BUFFER_WIDTH]; BUFFER_HEIGHT],
        }
    }
}