#![no_std]
#![no_main]

mod gdt;
mod vga;

use core::fmt::Write;
use core::panic::PanicInfo;
use gdt::GDT;
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
    println!("welcome to {}!", "rkfs-2");
    unsafe { GDT.load(); }

    println!("GDT loaded");
    loop {}
}
