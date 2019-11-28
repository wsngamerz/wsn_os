[org 0x7c00]
[bits 16]

section .text
    global main

; TODO: - Save boot drive into variable at the start
;       - Add header information describing filesystem

main:
    ; reset segment registers to 0
    cli
    jmp 0x0000:ZeroSeg
    ZeroSeg:
        xor ax, ax
        mov ss, ax
        mov ds, ax
        mov es, ax
        mov fs, ax
        mov gs, ax
        ; move stack pointer to main
        mov sp, main
        cld
    sti

    mov si, STR_INIT
    call printf
    
    ; reset disk
    push ax
    xor ax, ax
    int 0x13
    pop ax

    ; read disk    
    mov al, 10    ; no. sectors to read
    mov cl, 2    ; start sector pointer
    ; mov dl, 0x80 ; 0x80 = hdd, 0x00 = floppy (iso on usb emulates a floppy)
    call read_disk

    ; enable A20
    call enableA20

    ; jump to asm stored in second sector
    call second_sector
    
    ; force to hang
    jmp $ 

    ; include files
    %include "./printf.asm"
    %include "./printh.asm"
    %include "./read_disk.asm"
    %include "./testA20.asm"
    %include "./enableA20.asm"

    STR_INIT: db "Loading...", 0x0a, 0x0d, 0
    STR_A20_LOAD: db "A20 Pass", 0x0a, 0x0d, 0
    STR_A20_ERROR: db "A20 Err", 0x0a, 0x0d, 0
    STR_DISK_ERROR: db "Disk Err", 0x0a, 0x0d, 0


; padding and magic number for boot sector
times 510-($-$$) db 0
dw 0xaa55

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;      Second Sector      ;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;


second_sector:
    mov si, STR_LOADED
    call printf

    ; cause a beep on systems which support it
    mov si, BEEP
    call printf
    
    ; test for long mode compat
    call lm_check

    cli

    ; write 0's to memory
    mov edi, 0x1000 ; table start
    mov cr3, edi
    xor eax, eax
    mov ecx, 4096
    rep stosd
    mov edi, 0x1000

    ; Page map L4 table -> 0x1000
    ; Page map pointer table -> 0x2000
    ; Page directory table -> 0x3000
    ; Page table -> 0x4000

    mov dword [edi], 0x2003
    add edi, 0x1000
    mov dword [edi], 0x3003
    add edi, 0x1000
    mov dword [edi], 0x4003
    add edi, 0x1000

    mov dword ebx, 3
    mov ecx, 512

    .setEntry:
        mov dword [edi], ebx
        add ebx, 0x1000
        add edi, 8
        loop .setEntry

    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    mov ecx, 0xc0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    mov eax, cr0
    or eax, 1 << 31
    or eax, 1 << 0
    mov cr0, eax
    
    lgdt [GDT.Pointer]
    jmp GDT.Code:LongMode

    sti

    jmp $


%include "./lm_check.asm"
%include "./gdt.asm"

BEEP: db 0x07, 0
DEV: db "DEV", 0x0a, 0x0d, 0
STR_LOADED: db "Loaded second sector", 0x0a, 0x0d, 0
STR_LM_COMPAT: db "LM Pass", 0x0a, 0x0d, 0
STR_NO_LM: db "No LM", 0x0a, 0x0d, 0

[bits 64]
LongMode:
    cli ; disable interupts as we don't have IDT table

    VID_MEM equ 0xb8000

    ; create blue background
    mov edi, VID_MEM
    mov rax, 0x1f201f201f201f20
    mov ecx, 500
    rep stosq


    ; print out 'WSN OS Successfully Booted!'
    mov rax, 0x1f201f4e1f531f57
    mov [VID_MEM], rax
    mov rax, 0x1f421f201f531f4f
    mov [VID_MEM + 8], rax
    mov rax, 0x1f651f741f6f1f6f
    mov [VID_MEM + 16], rax
    mov rax, 0x1f751f531f201f64
    mov [VID_MEM + 24], rax
    mov rax, 0x1f731f651f631f63
    mov [VID_MEM + 32], rax
    mov rax, 0x1f6c1f751f661f73
    mov [VID_MEM + 40], rax
    mov rax, 0x1f201f211f791f6c
    mov [VID_MEM + 48], rax

    hlt
