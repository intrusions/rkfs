use super::segment_descriptor::{SegmentDescriptor, SegmentList};
use core::arch::asm;

pub const ENTRY_COUNT: usize = 7;

#[link_section = ".gdt"]
#[no_mangle]
pub static GDT: GlobalDescriptorTable = GlobalDescriptorTable {
    entry: [
        SegmentDescriptor::new(SegmentList::Null),
        SegmentDescriptor::new(SegmentList::KernelCode),
        SegmentDescriptor::new(SegmentList::KernelData),
        SegmentDescriptor::new(SegmentList::KernelStack),
        SegmentDescriptor::new(SegmentList::UserCode),
        SegmentDescriptor::new(SegmentList::UserData),
        SegmentDescriptor::new(SegmentList::UserStack),
    ],
};

#[repr(C, packed)]
pub struct GlobalDescriptorTableRegister {
    pub limit: u16,
    pub base: u32,
}

pub const GDTR: GlobalDescriptorTableRegister = GlobalDescriptorTableRegister {
    limit: (ENTRY_COUNT * 8 - 1) as u16,
    base: 0x00000800,
};

#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    pub entry: [SegmentDescriptor; ENTRY_COUNT],
}

impl GlobalDescriptorTable {
    pub unsafe fn load(&self) {
        asm!(
            "lgdt [{0}]",
            in(reg) &GDTR,
            options(nostack, preserves_flags),
        );

        const KERNEL_CODE_SELECTOR: u16 = 0x8;
        const KERNEL_DATA_SELECTOR: u16 = 0x10;
        const KERNEL_STACK_SELECTOR: u16 = 0x18;

        asm!(
            "push {code_sel}",
            "lea eax, [2f]",
            "push eax",
            "retf",
            "2:",
            "mov ds, {data_sel:x}",
            "mov es, {data_sel:x}",
            "mov fs, {data_sel:x}",
            "mov gs, {data_sel:x}",
            "mov ss, {stack_sel:x}",
            code_sel = const KERNEL_CODE_SELECTOR,
            data_sel = in(reg) KERNEL_DATA_SELECTOR,
            stack_sel = in(reg) KERNEL_STACK_SELECTOR,
            out("eax") _,
            options(nostack),
        );
    }
}
