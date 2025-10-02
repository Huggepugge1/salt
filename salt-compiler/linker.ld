ENTRY(main)

SECTIONS
{
    /* Load at 1 MB */
    . = 1M;

    /* Multiboot header from entry.o */
    .multiboot : { KEEP(*(.multiboot2_header)) }

    /* Code sections */
    .text : { multiboot.o(.text*); kernel.o(.text*) }

    /* Read-only data */
    .rodata : { kernel.o(.rodata*) }

    /* Writable data */
    .data : { kernel.o(.data*) }

    /* BSS */
    .bss : { kernel.o(.bss*) }
}
