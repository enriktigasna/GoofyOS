;; Loads second disk segment into memory at segment dh
disk_load:
    pusha
    push dx

    mov ah, 0x02
    mov al, dh
    
    mov ch, 0x00
    mov cl, 0x24

    mov dh, 0x02

    int 0x13
    jc disk_error

    pop dx
    cmp al, dh
    jne sectors_error

    popa
    ret

disk_error:
    jmp disk_error
    
sectors_error:
    jmp sectors_error