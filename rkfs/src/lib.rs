#![no_std]
#![no_main]

mod vga;
mod gdt;

use core::fmt::Write;
use core::panic::PanicInfo;
use gdt::GlobalDescriptorTable;
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
    println!("Welcome to {}!", "rkfs");

    let _gdt = GlobalDescriptorTable::new();

    let kernel_code_bits: u64 = unsafe {
        core::mem::transmute(_gdt.kernel_code)
    };
    println!("{:064b}", kernel_code_bits);

    loop {}
}
