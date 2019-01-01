ENTRY(_start)

SYS_STACK_SIZE = 1024;
IRQ_STACK_SIZE = 1024;
 
SECTIONS
{
    /* Starts at LOADER_ADDR. */
    . = 0x8000;
    __start = .;
    __text_start = .;
    .text :
    {
        KEEP(*(.text.boot))
        *(.text)
    }
    . = ALIGN(4096); /* align to page size */
    __text_end = .;
 
    __rodata_start = .;
    .rodata :
    {
        *(.rodata)
    }
    . = ALIGN(4096); /* align to page size */
    __rodata_end = .;
 
    __data_start = .;
    .data :
    {
        *(.data)
    }
    . = ALIGN(4096); /* align to page size */
    __data_end = .;
 
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss)
    }
    . = ALIGN(4096); /* align to page size */
    __bss_end = .;

    .stack : {
        __stack_start__ = .;

        . += SYS_STACK_SIZE;
        . = ALIGN(4);
        __sys_stack_end__ = .;

        . += IRQ_STACK_SIZE;
        . = ALIGN(4);
        __irq_stack_end__ = .;
    }

    . = ALIGN(4096);
    __kernel_heap_start__ = .;
    . = 0xFFFFFF;
    __kernel_heap_end__ = .;

    __end = .;

}
