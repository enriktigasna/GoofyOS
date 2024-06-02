bits 16
org 0x7c00

jmp short start
nop

%include "bootloader/bpb.inc"

start:
KERNEL_OFFSET equ 0x1000
mov [BOOT_DRIVE], dl

;; Set up stack 
mov bp, 0x9000
mov sp, bp

;; Load kernel into disk and switch to protected mode
call load_kernel
call switch_to_pm ;; This function will eventually call BEGIN_32 which will call the kernel

%include "bootloader/disk.asm"
%include "bootloader/gdt.asm"
%include "bootloader/protected-mode.asm"

[bits 16]
load_kernel:
    mov dh, 2
    mov dl, [BOOT_DRIVE]
    call disk_load
    ret

[bits 32]
BEGIN_32:
    call KERNEL_OFFSET
    jmp $


BOOT_DRIVE: db 0


times 510-($-$$) db 0
dw 0xaa55
