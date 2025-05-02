use super::screen_char::ScreenChar;
use volatile::Volatile;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    pub fn vga_buffer() -> &'static mut Self {
        unsafe { &mut *(0xb8000 as *mut Self) }
    }
}
