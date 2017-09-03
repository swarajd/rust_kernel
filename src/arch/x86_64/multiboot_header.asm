; info required:
; magic number      (0xE85250D6)
; architecture      (0 for i386, 4 for MIPS)
; header length     (total header size, including tags)
; checksum          -(magic + architecture + header_length)
; tags              (variable)
; end tag           (0, 0, 8)


section .multiboot_header
header_start:
    dd 0xe85250d6                ; magic number (multiboot 2)
    dd 0                         ; architecture 0 (protected mode i386)
    dd header_end - header_start ; header length
    ; checksum
    ; the hack used here is that the negative number doesn't fit, so we subtract it from 2^32
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; insert optional multiboot tags here

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:
