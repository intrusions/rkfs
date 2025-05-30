#![allow(dead_code)]

use crate::println;
use core::fmt::Write;

unsafe extern "C" {
    unsafe static stack_bottom: u8;
    unsafe static stack_top: u8;
}

pub unsafe fn dump_esp() {
    core::arch::asm!("push 0x13371337");
    
    println!(
        "{:#010X}",
        (read_esp() as *const u32).read_volatile()
    );
}

pub unsafe fn read_esp() -> usize {
    let esp: usize;
    
    core::arch::asm!(
        "mov {}, esp",
        out(reg) esp
    );
    esp
}

pub unsafe fn dump_stack_info() {
    let esp = read_esp();
    let top = &stack_top as *const u8 as usize;
    let bottom = &stack_bottom as *const u8 as usize;

    println!("--- Kernel Stack Dump ---");
    println!("[+] stack_bottom: {:#010X}", bottom);
    println!("[+] stack_top:    {:#010X}", top);
    println!("[+] esp:          {:#010X}", esp);
}

pub unsafe fn dump_stack() {
    let esp = read_esp();
    let top = &stack_top as *const u8 as usize;

    let stack_size = top - esp;
    let mut i: usize = 0;

    while i < stack_size {
        println!(
            "{:#010X}: {:#010X}",
            (esp + i),
            ((esp + i) as *const u32).read_volatile()
        );
        i += 1;
    }
}
