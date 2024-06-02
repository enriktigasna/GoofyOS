#![allow(dead_code)]

use super::color::ColorCode;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Char {
    pub character: u8,
    pub color: ColorCode,
}

pub enum CharError {
    BufferTooSmall,
}

impl Char {
    pub fn from_string(s: &str, color: ColorCode, buffer: &mut [Char]) -> Result<(), CharError> {
        if s.len() > buffer.len() {
            return Err(CharError::BufferTooSmall);
        }
        for (i, c) in s.chars().enumerate() {
            buffer[i] = Char {
                character: c as u8,
                color,
            };
        }
        Ok(())
    }
}