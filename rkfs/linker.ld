ENTRY(_start)

SECTIONS
{
	. = 0x00000800;
	.gdt :
	{
		KEEP
		(
			*(.gdt)
		)
	}

	. = 1M;
	.text :
	{
		*(.multiboot)
		*(.text)
	}

	.rodata :
	{
		*(.rodata)
	}

	.data :
	{
		*(.data)
	}

	.bss :
	{
		*(COMMON)
		*(.bss)
	}
}
