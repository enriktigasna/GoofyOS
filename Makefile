BUILD_DIR = build

all: $(BUILD_DIR)/bootloader.bin

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(BUILD_DIR)/bootloader.bin: mbr.asm | $(BUILD_DIR)
	nasm -f bin mbr.asm -o $(BUILD_DIR)/bootloader.bin

clean:
	rm -rf $(BUILD_DIR)

