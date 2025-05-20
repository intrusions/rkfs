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
        WRITER
            .lock()
            .set_color(ColorCode::new(Color::Yellow, Color::Black));
    }

    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    println!("{}", "42");

    {
        WRITER
            .lock()
            .set_color(ColorCode::new(Color::Green, Color::Black));
    }

    println!("{} in green", "42");

    loop {}
}
