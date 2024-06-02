;; Loads memory from 17th sector from floppy drive (adress 0x4200) to memory adress 0x1000 (KERNEL_OFFSET)
disk_load:
    pusha
    mov bx, KERNEL_OFFSET  ;; Load it into KERNEL_OFFSET
    ;; Fun fact: took me a whole day to figure out that I was supposed to use the offset instead of the segment
    ;; I was trying to load it into 0x1000:0x0000 instead of 0x0000:0x1000
    ;; Thank you bochs debugger

    mov ah, 0x02    ;; Read sectors
    mov al,        ;; Read 4 sectors
    
    mov ch, 0x00    ;; Grab Cylinder 0
    mov cl, 15      ;; Grab Sector 15
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