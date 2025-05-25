#![no_std]
#![no_main]

mod gdt;
mod vga;

use core::fmt::Write;
use core::panic::PanicInfo;
use gdt::GlobalDescriptorTable;
use vga::{
    color::{Color, ColorCode},
    writer::WRITER,
};

pub fn print_segments() {
    let cs: u16;
    let ds: u16;
    let es: u16;
    let fs: u16;
    let gs: u16;
    let ss: u16;

    unsafe {
        core::arch::asm!(
            "mov {0:x}, cs",
            "mov {1:x}, ds",
            "mov {2:x}, es",
            "mov {3:x}, fs",
            "mov {4:x}, gs",
            "mov {5:x}, ss",
            out(reg) cs,
            out(reg) ds,
            out(reg) es,
            out(reg) fs,
            out(reg) gs,
            out(reg) ss,
            options(nomem, nostack, preserves_flags),
        );
    }

    println!("  CS = {:#X}", cs);
    println!("  DS = {:#X}", ds);
    println!("  ES = {:#X}", es);
    println!("  FS = {:#X}", fs);
    println!("  GS = {:#X}", gs);
    println!("  SS = {:#X}", ss);
}

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

    let gdt = GlobalDescriptorTable::new();
    unsafe { gdt.load_gdt(); }

    {
        print_segments();
    }

    loop {}
}
