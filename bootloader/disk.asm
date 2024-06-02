;; Loads memory from 17th sector from floppy drive (adress 0x4200) to memory adress 0x1000 (KERNEL_OFFSET)
disk_load:
    pusha
    mov bx, KERNEL_OFFSET  ;; Load it into KERNEL_OFFSET

    mov ah, 0x02    ;; Read sectors
    mov al, 4       ;; Read 4 sectors
    
    mov ch, 0x00    ;; Grab Cylinder 0
    mov cl, 16      ;; Grab Sector 16
    mov dh, 0x01    ;; Grab Head 1

    int 0x13        ;; Call disk interrupt
    jc disk_error

    cmp al, 4
    jne sectors_error

    popa
    ret

disk_error:
    jmp disk_error
    
sectors_error:
    jmp sectors_error