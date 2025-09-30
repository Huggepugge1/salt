; kernel.asm
BITS 32
section .multiboot
align 4
multiboot_header:
    dd 0x1BADB002          ; magic number
    dd 0                    ; flags
    dd -(0x1BADB002)        ; checksum
