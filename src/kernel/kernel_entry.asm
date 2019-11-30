global _start
[bits 64]


_start:
    [extern kernel_main]
    call kernel_main
    jmp $
