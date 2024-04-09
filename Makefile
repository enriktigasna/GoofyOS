BUILD_DIR = build
BOOTLOADER_DIR = bootloader
KERNEL_DIR = kernel
KERNEL_OBJECTS = $(BUILD_DIR)/kernel-entry.o \
$(BUILD_DIR)/kernel.o \
$(BUILD_DIR)/vga_video.o \
$(BUILD_DIR)/console.o

all: $(BUILD_DIR)/os-image.img


$(BUILD_DIR)/mbr.bin: $(BOOTLOADER_DIR)/mbr.asm
	nasm -f bin $< -o $@

$(BUILD_DIR)/%.o: $(KERNEL_DIR)/%.c
	gcc -fno-pie -m32 -ffreestanding -c $< -o $@

$(BUILD_DIR)/kernel-entry.o: $(KERNEL_DIR)/kernel-entry.asm
	nasm -f elf $< -o $@

$(BUILD_DIR)/kernel.bin: $(KERNEL_OBJECTS)
	ld -m elf_i386 -o $@ -Ttext 0x1000 $^ --oformat binary

$(BUILD_DIR)/os-image.img: $(BUILD_DIR)/mbr.bin $(BUILD_DIR)/kernel.bin
	dd if=/dev/zero of=$(BUILD_DIR)/os-image.img bs=512 count=8320
	mkfs.fat -F 32 -s 1 -n "GOOFYOS" $(BUILD_DIR)/os-image.img
	dd if=$(BUILD_DIR)/mbr.bin of=$(BUILD_DIR)/os-image.img conv=notrunc
	dd if=$(BUILD_DIR)/mbr.bin of=$(BUILD_DIR)/os-image.img seek=6 conv=notrunc
	mcopy -i $(BUILD_DIR)/os-image.img $(BUILD_DIR)/kernel.bin "::kernel.bin"


clean:
	rm -rf $(BUILD_DIR)/*.bin $(BUILD_DIR)/*.o