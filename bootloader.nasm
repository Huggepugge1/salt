[bits 16]
org 0x7c00
cli
xor ax, ax
mov ds, ax
mov ss, ax
mov sp, 0x7c00
jmp 0x1000
