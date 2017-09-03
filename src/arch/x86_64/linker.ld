ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        /* not used anywhere, and is trashed due to --gc-sections, but we explicitly keep */
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text)
    }
}