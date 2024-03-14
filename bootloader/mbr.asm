bits 16
org 0x7c00

;;; JUMP TO BOOT SECTOR
jmp short start
nop ; pad out to 3 bytes

;;; BPB
OEMLabel db "GOOFY-OS"
BytesPerSector dw 512
SectorsPerCluster db 1
ReservedSectors dw 0x20
NumberOfFATs db 2
RootEntries dw 0
NumberOfSectors dw 0
MediaDescriptor db 0xf8
SectorsPerFAT dw 0
SectorsPerTrack dw 0x3d
SectorsPerHead dw 2
HiddenSectors dd 0
TotalSectors dd 0x2000 ;; 4 MB
BigSectorsPerFAT dd 64
Flags dw 0
FSVersion dw 0
RootDirectoryStart dd 160
FSInfoSector dw 1
BackupBootSector dw 6

TIMES 12 db 0 ;; Reserved

DriveNumber db 0x00
ReservedByte db 0
Signature db 0x29
VolumeID dd 0x12345678
VolumeLabel db "GOOFYOS    "
FileSystem db "FAT32   "

;;; BOOT SECTOR
start:
KERNEL_OFFSET equ 0x1000
mov [BOOT_DRIVE], dl

;; Set up stack
mov bp, 0x9000
mov sp, bp

call load_kernel
call switch_to_pm

%include "bootloader/disk.asm"
%include "bootloader/gdt.asm"
%include "bootloader/protected-mode.asm"

[bits 16]
load_kernel:
    mov bx, KERNEL_OFFSET
    mov dh, 2
    mov dl, [BOOT_DRIVE]
    call disk_load
    ret

[bits 32]
BEGIN_32:
    call KERNEL_OFFSET
    jmp $

BOOT_DRIVE:
  db 0

times 510-($-$$) db 0
dw 0xaa55
