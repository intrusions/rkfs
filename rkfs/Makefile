TARGET          := i386
TARGET_TRIPLE   := $(TARGET)-unknown-none
TARGET_PATH     := arch/$(TARGET)/$(TARGET_TRIPLE).json

KERNEL_NAME     := rkfs
KERNEL_BIN      := $(KERNEL_NAME).bin
KERNEL_PATH     := build/iso/boot/$(KERNEL_BIN)
ISO_PATH        := $(KERNEL_NAME).iso

CARGO           := cargo
RUSTC           := rustc
NASM            := nasm
LD              := ld
QEMU            := qemu-system-$(TARGET)

ASM_SRCS        := src/boot/boot.asm
ASM_OBJS        := $(ASM_SRCS:.asm=.o)

CARGO_OBJS      := target/$(TARGET_TRIPLE)/release/librkfs.a
LD_FILE         := linker.ld

RUSTC_FLAGS     := -- --emit=obj
CARGO_FLAGS     := --release
NASM_FLAGS      := -f elf32
LD_FLAGS        := -m elf_$(TARGET) -T $(LD_FILE)

.DEFAULT_GOAL   := build-iso

.PHONY: all
all: build-iso

.PHONY: build-bin
build-bin: $(ASM_SRCS) $(LD_FILE)
	$(CARGO) build $(CARGO_FLAGS)
	$(NASM) $(NASM_FLAGS) -o $(ASM_OBJS) $(ASM_SRCS)
	$(LD) $(LD_FLAGS) -o $(KERNEL_BIN) $(ASM_OBJS) $(CARGO_OBJS)
	mv $(KERNEL_BIN) $(KERNEL_PATH)

.PHONY: build-iso
build-iso: build-bin
	grub-mkrescue --compress=xz -o $(ISO_PATH) build/iso

.PHONY: run
run:
	$(QEMU) -cdrom $(ISO_PATH) -boot menu=on

.PHONY: is-multiboot
is-multiboot:
	@if grub-file --is-x86-multiboot $(KERNEL_PATH); then	\
		echo "Multiboot: YES"; 								\
	else 													\
		echo "Multiboot: NO";		 						\
	fi

.PHONY: size
size:
	@ls -l $(ISO_PATH)

.PHONY: clean
clean:
	cargo clean
	rm -f $(ASM_OBJS)

.PHONY: fclean
fclean: clean
	rm -f $(KERNEL_PATH) $(ISO_PATH)

.PHONY: re
re: fclean all
