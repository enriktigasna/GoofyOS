bits 16
org 0x7c00

;; What memory region kernel will be loaded in
KERNEL_OFFSET equ 0x1000

;; BIOS stores boot drive in dl register
mov [BOOT_DRIVE], dl

;; Set up stack pointer
;; Stack grows downwards from 0x9000
mov bp, 0x9000
mov sp, bp

call load_kernel
call switch_to_32bit

jmp $

%include "disk.asm"
%include "gdt.asm"
%include "protected_mode.asm"

load_kernel:
  mov bx, KERNEL_OFFSET
  mov dh, 2
  mov dl, BOOT_DRIVE
  call disk_load
  ret

bits 32
BEGIN_32:
  call KERNEL_OFFSET
  jmp $

;; Storing default boot drive here
;; Should be put here at beginning of mbr execution from dl register
BOOT_DRIVE:
  db 0

;; Magic number for boot sector after padding rest of sector with 0s
times 510 - ($-$$) db 0
dw 0xaa55