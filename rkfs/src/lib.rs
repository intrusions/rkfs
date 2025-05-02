#![no_std]
#![no_main]

mod vga;
use core::fmt::Write;
use core::panic::PanicInfo;

use vga::{
    color::{Color, ColorCode},
    writer::WRITER,
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    {
        let mut writer = WRITER.lock();
        writer.set_color(ColorCode::new(Color::Yellow, Color::Black));
    }

    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    println!("Hello from {}", "rkfs");

    {
        let mut writer = WRITER.lock();
        writer.set_color(ColorCode::new(Color::Green, Color::Black));
    }

    println!("this is so green!!");
    loop {}
}
