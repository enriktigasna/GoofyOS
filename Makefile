BUILD_DIR = build
BOOTLOADER_DIR = bootloader
BOOTLOADER_2_DIR = bootloader-2
GOOFYBOOT_LINKER = goofyboot.ld
KERNEL_DIR = kernel
RUST_TARGET_SPEC = i386_kernel

# Kernel binary
GOOFYBOOT_BIN = $(BUILD_DIR)/goofyboot.bin
KERNEL_BIN = $(BUILD_DIR)/vmgoofuz

# Disk image file
DISK_IMG = $(BUILD_DIR)/os-image.img

# Targets
all: $(DISK_IMG)

# Build MBR bootloader
$(BUILD_DIR)/mbr.bin: $(BOOTLOADER_DIR)/mbr.asm
	nasm -f bin $< -o $@

# Build second stage bootloader
$(BUILD_DIR)/goofyboot_entry.o: $(BOOTLOADER_2_DIR)/goofyboot_entry.asm
	nasm -f elf $< -o $@

$(BUILD_DIR)/goofyboot.o: $(BOOTLOADER_2_DIR)/goofyboot.c
	gcc -fno-pie -m32 -ffreestanding -c $< -o $@

$(GOOFYBOOT_BIN): $(BUILD_DIR)/goofyboot_entry.o $(BUILD_DIR)/goofyboot.o
	ld -m elf_i386 -o $@ -T $(GOOFYBOOT_LINKER) $^ --oformat binary

# Build Rust code into a static library
$(BUILD_DIR)/libkernel.a:
	@cd $(KERNEL_DIR) && cargo build --release --target $(RUST_TARGET_SPEC).json
	cp $(KERNEL_DIR)/target/$(RUST_TARGET_SPEC)/release/libkernel.a $@

# Link the final kernel binary from Rust static library
$(KERNEL_BIN): $(BUILD_DIR)/libkernel.a
	ld -m elf_i386 -o $@ -Ttext 0x1000 $(BUILD_DIR)/libkernel.a --oformat binary

# Create the OS disk image
$(DISK_IMG): $(BUILD_DIR)/mbr.bin $(GOOFYBOOT_BIN) $(KERNEL_BIN)
	dd if=/dev/zero of=$@ bs=512 count=2880
	mkfs.fat -F 12 -s 1 -n "GOOFYOS" $@
	dd if=$(BUILD_DIR)/mbr.bin of=$@ conv=notrunc
	mcopy -i $@ $(GOOFYBOOT_BIN) "::goofyboot"
#	mcopy -i $@ $(KERNEL_BIN) "::vmgoofuz"

# Clean up
clean:
	rm -rf $(BUILD_DIR)/*.bin $(BUILD_DIR)/*.o $(BUILD_DIR)/*.a
	cd $(KERNEL_DIR) && cargo clean

clean_build:
	rm -rf $(BUILD_DIR)/*