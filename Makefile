BUILD_DIR = build
BOOTLOADER_DIR = bootloader
KERNEL_DIR = kernel
RUST_TARGET_SPEC = i386_kernel

# Kernel binary
KERNEL_BIN = $(BUILD_DIR)/vmgoofuz

# Disk image file
DISK_IMG = $(BUILD_DIR)/os-image.img

# Targets
all: $(DISK_IMG)

# Build MBR bootloader
$(BUILD_DIR)/mbr.bin: $(BOOTLOADER_DIR)/mbr.asm
	nasm -f bin $< -o $@

# Build Rust code into a static library
$(BUILD_DIR)/libkernel.a:
	@cd $(KERNEL_DIR) && cargo build --release --target $(RUST_TARGET_SPEC).json
	cp $(KERNEL_DIR)/target/$(RUST_TARGET_SPEC)/release/libkernel.a $@

# Link the final kernel binary from Rust static library
$(KERNEL_BIN): $(BUILD_DIR)/libkernel.a
	ld -m elf_i386 -o $@ -Ttext 0x1000 $(BUILD_DIR)/libkernel.a --oformat binary

# Create the OS disk image
$(DISK_IMG): $(BUILD_DIR)/mbr.bin $(KERNEL_BIN)
	dd if=/dev/zero of=$@ bs=512 count=2880
	mkfs.fat -F 12 -s 1 -n "GOOFYOS" $@
	dd if=$(BUILD_DIR)/mbr.bin of=$@ conv=notrunc
	mcopy -i $@ $(KERNEL_BIN) "::vmgoofuz"

# Clean up
clean:
	rm -rf $(BUILD_DIR)/*.bin $(BUILD_DIR)/*.o $(BUILD_DIR)/*.a
	cd $(KERNEL_DIR) && cargo clean

clean_build:
	rm -rf $(BUILD_DIR)/*