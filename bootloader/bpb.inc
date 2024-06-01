;; Go to https://wiki.osdev.org/FAT to learn more

;; BPB
OEMLabel db "GOOFY-OS"
BytesPerSector dw 512
SectorsPerCluster db 1
ReservedSectors dw 1
NumberOfFATs db 2
RootEntries dw 224
TotalSectors dw 2880
MediaDescriptor db 0xf0
SectorsPerFAT dw 9
SectorsPerTrack dw 18
SectorsPerHead dw 2
HiddenSectors dd 0
BigTotalSectors dd 0
;; End of BPB

TIMES 12 db 0 ;; Reserved

DriveNumber db 0x00
ReservedByte db 0
Signature db 0x29
VolumeID dd 0x12345678
VolumeLabel db "GOOFYOS    "
FileSystem db "FAT12   "