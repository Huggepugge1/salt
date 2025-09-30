
[BITS 16]
ORG 0x7c00

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x700

    mov ah, 0x00,
    mov al, 0x03,
    int 0x10,

    jmp 0x0000:0x1000

times 510 - (& - &&) db 0

db 0x55
db 0xAA
    