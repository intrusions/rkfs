use core::arch::asm;

#[allow(dead_code)]
pub enum SegmentList {
    Null,
    KernelCode,
    KernelData,
    UserCode,
    UserData,
}

#[allow(dead_code)]
#[repr(u8)]
pub enum Granularity {
    Byte = 0,
    Page4K = 1,
}

#[allow(dead_code)]
#[repr(u8)]
pub enum DescriptorType {
    System = 0,
    CodeOrData = 1,
}

#[allow(dead_code)]
#[repr(u8)]
pub enum LongMode {
    Disabled = 0,
    Enabled = 1,
}

#[allow(dead_code)]
#[repr(u8)]
pub enum SegmentType {
    DataReadOnly = 0b0000,
    DataReadOnlyAccessed = 0b0001,
    DataReadWrite = 0b0010,
    DataReadWriteAccessed = 0b0011,
    DataReadOnlyExpandDown = 0b0100,
    DataReadOnlyExpandDownAccessed = 0b0101,
    DataReadWriteExpandDown = 0b0110,
    DataReadWriteExpandDownAccessed = 0b0111,

    CodeExecuteOnly = 0b1000,
    CodeExecuteOnlyAccessed = 0b1001,
    CodeExecuteRead = 0b1010,
    CodeExecuteReadAccessed = 0b1011,
    CodeExecuteOnlyConforming = 0b1100,
    CodeExecuteOnlyConformingAccessed = 0b1101,
    CodeExecuteReadConforming = 0b1110,
    CodeExecuteReadConformingAccessed = 0b1111,
}

#[allow(dead_code)]
#[repr(u8)]
pub enum DescriptorPrivilegeLevel {
    Ring0 = 0b00,
    Ring1 = 0b01,
    Ring2 = 0b10,
    Ring3 = 0b11,
}

#[allow(dead_code)]
#[repr(u8)]
pub enum SegmentPresent {
    NotPresent = 0,
    Present = 1,
}

#[allow(dead_code)]
#[repr(u8)]
pub enum DefaultOperationSize {
    Bit16 = 0,
    Bit32 = 1,
}

#[allow(dead_code)]
#[repr(u8)]
pub enum AvailableBit {
    NotAvailable = 0,
    Available = 1,
}

#[repr(C, packed)]
pub struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl SegmentDescriptor {
    pub fn new(segment_type: SegmentList) -> Self {
        match segment_type {
            SegmentList::Null => Self::create_null_segment(),
            SegmentList::KernelCode => Self::create_kernel_code_segment(),
            SegmentList::KernelData => Self::create_kernel_data_segment(),
            SegmentList::UserCode => Self::create_user_code_segment(),
            SegmentList::UserData => Self::create_user_data_segment(),
        }
    }

    fn create_null_segment() -> SegmentDescriptor {
        SegmentDescriptor {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        }
    }

    fn create_kernel_code_segment() -> SegmentDescriptor {
        let base: u32 = 0x00000000;
        let limit: u32 = 0x000FFFFF;

        let access: u8 = (SegmentPresent::Present as u8) << 7
            | ((DescriptorPrivilegeLevel::Ring0 as u8) << 5)
            | ((DescriptorType::CodeOrData as u8) << 4)
            | (SegmentType::CodeExecuteRead as u8);

        let granularity: u8 = (Granularity::Byte as u8) << 7
            | ((DefaultOperationSize::Bit32 as u8) << 6)
            | ((LongMode::Disabled as u8) << 5)
            | ((AvailableBit::NotAvailable as u8) << 4)
            | ((limit >> 16) & 0x0F) as u8;

        SegmentDescriptor {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_mid: ((base >> 16) & 0xFF) as u8,
            access,
            granularity,
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }

    fn create_kernel_data_segment() -> SegmentDescriptor {
        let base: u32 = 0x00000000;
        let limit: u32 = 0x000FFFFF;

        let access: u8 = (SegmentPresent::Present as u8) << 7
            | ((DescriptorPrivilegeLevel::Ring0 as u8) << 5)
            | ((DescriptorType::CodeOrData as u8) << 4)
            | (SegmentType::DataReadWrite as u8);

        let granularity: u8 = (Granularity::Byte as u8) << 7
            | ((DefaultOperationSize::Bit32 as u8) << 6)
            | ((LongMode::Disabled as u8) << 5)
            | ((AvailableBit::NotAvailable as u8) << 4)
            | ((limit >> 16) & 0x0F) as u8;

        SegmentDescriptor {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_mid: ((base >> 16) & 0xFF) as u8,
            access,
            granularity,
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }

    fn create_user_code_segment() -> SegmentDescriptor {
        let base: u32 = 0x00000000;
        let limit: u32 = 0x000FFFFF;

        let access: u8 = (SegmentPresent::Present as u8) << 7
            | ((DescriptorPrivilegeLevel::Ring3 as u8) << 5)
            | ((DescriptorType::CodeOrData as u8) << 4)
            | (SegmentType::CodeExecuteRead as u8);

        let granularity: u8 = (Granularity::Byte as u8) << 7
            | ((DefaultOperationSize::Bit32 as u8) << 6)
            | ((LongMode::Disabled as u8) << 5)
            | ((AvailableBit::NotAvailable as u8) << 4)
            | ((limit >> 16) & 0x0F) as u8;

        SegmentDescriptor {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_mid: ((base >> 16) & 0xFF) as u8,
            access,
            granularity,
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }

    fn create_user_data_segment() -> SegmentDescriptor {
        let base: u32 = 0x00000000;
        let limit: u32 = 0x000FFFFF;

        let access: u8 = (SegmentPresent::Present as u8) << 7
            | ((DescriptorPrivilegeLevel::Ring3 as u8) << 5)
            | ((DescriptorType::CodeOrData as u8) << 4)
            | (SegmentType::DataReadWrite as u8);

        let granularity: u8 = (Granularity::Byte as u8) << 7
            | ((DefaultOperationSize::Bit32 as u8) << 6)
            | ((LongMode::Disabled as u8) << 5)
            | ((AvailableBit::NotAvailable as u8) << 4)
            | ((limit >> 16) & 0x0F) as u8;

        SegmentDescriptor {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_mid: ((base >> 16) & 0xFF) as u8,
            access,
            granularity,
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    entry: [SegmentDescriptor; 5],
}

impl GlobalDescriptorTable {
    pub fn new() -> Self {
        GlobalDescriptorTable {
            entry: [
                SegmentDescriptor::new(SegmentList::Null),
                SegmentDescriptor::new(SegmentList::KernelCode),
                SegmentDescriptor::new(SegmentList::KernelData),
                SegmentDescriptor::new(SegmentList::UserCode),
                SegmentDescriptor::new(SegmentList::UserData),
            ],
        }
    }

    pub unsafe fn load_gdt(&self) {
        let gdtr = self.get_gdtr();

        asm!(
            "lgdt [{0}]",
            in(reg) &gdtr,
            options(nostack, preserves_flags),
        );

        const KERNEL_CODE_SELECTOR: u16 = 0x08;
        const KERNEL_DATA_SELECTOR: u16 = 0x10;

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
            "mov ss, {data_sel:x}",
            code_sel = const KERNEL_CODE_SELECTOR,
            data_sel = in(reg) KERNEL_DATA_SELECTOR,
            out("eax") _,
            options(nostack),
        );
    }

    fn get_gdtr(&self) -> GlobalDescriptorTableRegister {
        GlobalDescriptorTableRegister {
            limit: (core::mem::size_of::<[SegmentDescriptor; 5]>() - 1) as u16,
            base: self.entry.as_ptr() as u32,
        }
    }
}

#[repr(C, packed)]
struct GlobalDescriptorTableRegister {
    limit: u16,
    base: u32,
}
