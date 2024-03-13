BUILD_DIR = build
BOOTLOADER_DIR = bootloader
KERNEL_DIR = kernel

all: $(BUILD_DIR)/os-image.bin


$(BUILD_DIR)/mbr.bin: $(BOOTLOADER_DIR)/mbr.asm
	nasm -f bin $< -o $@

$(BUILD_DIR)/kernel.o: $(KERNEL_DIR)/kernel.c
	gcc -fno-pie -m32 -ffreestanding -c $< -o $@

$(BUILD_DIR)/kernel-entry.o: $(KERNEL_DIR)/kernel-entry.asm
	nasm -f elf $< -o $@

$(BUILD_DIR)/kernel.bin: $(BUILD_DIR)/kernel-entry.o $(BUILD_DIR)/kernel.o
	ld -m elf_i386 -o $@ -Ttext 0x1000 $^ --oformat binary

$(BUILD_DIR)/os-image.bin: $(BUILD_DIR)/mbr.bin $(BUILD_DIR)/kernel.bin
	cat $^ > $@

clean:
	rm -rf $(BUILD_DIR)/*.bin $(BUILD_DIR)/*.o