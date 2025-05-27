pub enum SegmentList {
    Null,
    KernelCode,
    KernelData,
    KernelStack,
    UserCode,
    UserData,
    UserStack,
}

#[allow(dead_code)]
#[repr(u8)]
enum Granularity {
    Byte = 0,
    Page4K = 1,
}

#[allow(dead_code)]
#[repr(u8)]
enum DescriptorType {
    System = 0,
    CodeOrData = 1,
}

#[allow(dead_code)]
#[repr(u8)]
enum LongMode {
    Disabled = 0,
    Enabled = 1,
}

#[allow(dead_code)]
#[repr(u8)]
enum SegmentType {
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
enum DescriptorPrivilegeLevel {
    Ring0 = 0b00,
    Ring1 = 0b01,
    Ring2 = 0b10,
    Ring3 = 0b11,
}

#[allow(dead_code)]
#[repr(u8)]
enum SegmentPresent {
    NotPresent = 0,
    Present = 1,
}

#[allow(dead_code)]
#[repr(u8)]
enum DefaultOperationSize {
    Bit16 = 0,
    Bit32 = 1,
}

#[allow(dead_code)]
#[repr(u8)]
enum AvailableBit {
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
    pub const fn new(segment_type: SegmentList) -> Self {
        match segment_type {
            SegmentList::Null => Self::create_null_segment(),
            SegmentList::KernelCode => Self::create_kernel_code_segment(),
            SegmentList::KernelData => Self::create_kernel_data_segment(),
            SegmentList::KernelStack => Self::create_kernel_stack_segment(),
            SegmentList::UserCode => Self::create_user_code_segment(),
            SegmentList::UserData => Self::create_user_data_segment(),
            SegmentList::UserStack => Self::create_user_stack_segment(),
        }
    }

    const fn create_segment(
        base: u32,
        limit: u32,
        dpl: DescriptorPrivilegeLevel,
        seg_type: SegmentType,
    ) -> Self {
        let access: u8 = (SegmentPresent::Present as u8) << 7
            | ((dpl as u8) << 5)
            | ((DescriptorType::CodeOrData as u8) << 4)
            | (seg_type as u8);

        let granularity: u8 = (Granularity::Page4K as u8) << 7
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

    const fn create_null_segment() -> SegmentDescriptor {
        SegmentDescriptor {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        }
    }

    const fn create_kernel_code_segment() -> Self {
        Self::create_segment(
            0x00000000,
            0x000FFFFF,
            DescriptorPrivilegeLevel::Ring0,
            SegmentType::CodeExecuteRead,
        )
    }

    const fn create_kernel_data_segment() -> Self {
        Self::create_segment(
            0x00000000,
            0x000FFFFF,
            DescriptorPrivilegeLevel::Ring0,
            SegmentType::DataReadWrite,
        )
    }

    const fn create_kernel_stack_segment() -> Self {
        Self::create_segment(
            0x00000000,
            0x000FFFFF,
            DescriptorPrivilegeLevel::Ring0,
            SegmentType::DataReadWrite,
        )
    }

    const fn create_user_code_segment() -> Self {
        Self::create_segment(
            0x00000000,
            0x000FFFFF,
            DescriptorPrivilegeLevel::Ring3,
            SegmentType::CodeExecuteRead,
        )
    }

    const fn create_user_data_segment() -> Self {
        Self::create_segment(
            0x00000000,
            0x000FFFFF,
            DescriptorPrivilegeLevel::Ring3,
            SegmentType::DataReadWrite,
        )
    }

    const fn create_user_stack_segment() -> Self {
        Self::create_segment(
            0x00000000,
            0x000FFFFF,
            DescriptorPrivilegeLevel::Ring3,
            SegmentType::DataReadWrite,
        )
    }
}
