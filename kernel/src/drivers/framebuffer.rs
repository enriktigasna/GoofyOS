use limine::request::FramebufferRequest;

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub bpp: u16,
    pub pointer: *mut u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Debug)]
pub enum FramebufferError {
    NoFramebuffer,
    ResponseError,
}

impl Framebuffer {
    pub fn new() -> Result<Self, FramebufferError> {
        let framebuffer_response = FRAMEBUFFER_REQUEST.get_response().unwrap();
        if framebuffer_response.framebuffers().count() < 1 {
            return Err(FramebufferError::NoFramebuffer);
        }
        let framebuffer = framebuffer_response
            .framebuffers()
            .next()
            .ok_or(FramebufferError::ResponseError)?;

        Ok(Framebuffer {
            width: framebuffer.width() as usize,
            height: framebuffer.height() as usize,
            pitch: framebuffer.pitch() as usize,
            bpp: framebuffer.bpp(),
            pointer: framebuffer.addr(),
        })
    }

    pub fn set_pixel(&self, x: usize, y: usize, color: &Color) {
        let pixel_offset: usize = (y * self.pitch) + x * 4;
        unsafe {
            *(self.pointer.add(pixel_offset) as *mut Color) = *color;
        }
        return;
    }

    pub fn draw_bitmap_8x16(&self, x: usize, y: usize, fg: &Color, bg: &Color, buf: &[u8]) {
        for row in 0..16 {
            for col in 0..8 {
                let bit_index = row * 8 + col;
                let byte_index = bit_index / 8;
                let bit_offset = 7 - (bit_index % 8);

                match (buf[byte_index] >> bit_offset) & 1 {
                    1 => self.set_pixel(x + col, y + row, fg),
                    _ => self.set_pixel(x + col, y + row, bg),
                }
            }
        }
    }
}
