;; Black magic to switch to 32-bit mode

[bits 16]
switch_to_32bit:
    cli                     ; 1. disable interrupts
    lgdt [GDT_Descriptor]   ; 2. load GDT descriptor
    mov eax, cr0
    or eax, 0x1             ; 3. enable protected mode
    mov cr0, eax
    jmp CODE_SEG:init_32bit ; 4. far jump

[bits 32]
init_32bit:
    mov ax, DATA_SEG
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    
    call BEGIN_32           ; 7. move back to mbr.asm