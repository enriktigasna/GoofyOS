bits 16

org 0x7c00


;; Clear screen
mov ah, 0x06
mov bh, 0x09

xor al, al ; Both are set to 0
xor cx, cx

mov dx, 0x184f
mov bh, 0x09
int 0x10

;; Set cursor to 0
mov ah, 0x02
xor dh, dh
xor dl, dl
xor bh, bh
int 0x10


mov si, greeting
mov ah, 0x0e
loop:
  lodsb
  test al, al
  jz end

  int 0x10
  jmp loop
end:
  hlt

;; Write Greeting
greeting:
  db "== Welcome to GoofyOS =="

padding:
  db "========================"


times 510 - ($-$$) db 0
dw 0xaa55
