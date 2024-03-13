GDT_Start:
    GDT_Null:
        dd 0
        dd 0
    GDT_Code:
        dw 0xffff
        dw 0
        db 0
        db 10011010b
        db 11001111b
        db 0
    GDT_Data:
        dw 0xffff
        dw 0
        db 0
        db 10010010b
        db 11001111b
        db 0
GDT_End:

GDT_Descriptor:
    dw GDT_End - GDT_Start - 1
    dd GDT_Start

CODE_SEG equ GDT_Code - GDT_Start
DATA_SEG equ GDT_Data - GDT_Start