use core::fmt::{Result, Write};
use lazy_static::lazy_static;
use spin::Mutex;

use super::buffer::{Buffer, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};
use super::color::{Color, ColorCode};
// use super::screen_char::ScreenChar;

pub struct Writer {
    col: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(color_code: ColorCode) -> Self {
        Self {
            col: 0,
            color_code,
            buffer: Buffer::vga_buffer(),
        }
    }

    fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.new_line();
        } else {
            if self.col >= BUFFER_WIDTH {
                self.new_line();
            }

            let row = BUFFER_HEIGHT - 1;
            let col = self.col;
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_character: byte,
                color_code: self.color_code,
            });

            self.col += 1;
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let ch = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(ch);
            }
        }

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[BUFFER_HEIGHT - 1][col].write(ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            });
        }

        self.col = 0;
    }

    pub fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    pub fn write_str_f(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_str_f(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> =
        Mutex::new(Writer::new(ColorCode::new(Color::White, Color::Black)));
}
