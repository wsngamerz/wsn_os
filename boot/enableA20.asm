enableA20:
    pusha

    ;; check if already enabled
    call testA20
    cmp ax, 1
    je .done

    ;; BIOS enable A20
    mov ax, 0x2401
    int 0x15

    call testA20
    cmp ax, 1
    je .done

    ;; keyboard enable A20
    cli

    call .waitC
    mov al, 0xad
    out 0x64, al

    call .waitC
    mov al, 0xd0
    out 0x64, al

    call .waitD
    in al, 0x60
    push ax

    call .waitC
    mov al, 0xd1
    out 0x64, al

    call .waitC
    pop ax
    or al, 2
    out 0x60, al

    call .waitC
    mov al, 0xae
    out 0x64, al

    call .waitC
    sti

    call testA20
    cmp ax, 1
    je .done

    ;; fast A20
    in al, 0x92
    or al, 2
    out 0x92, al

    call testA20
    cmp al, 1
    je .done

    popa
    mov si, STR_A20_ERROR
    call printf
    
    jmp $

    .done:
        popa
        mov si, STR_A20_LOAD
        call printf
        ret

    .waitC:
        in al, 0x64
        test al, 2
        jnz .waitC
        ret

    .waitD:
        in al, 0x64
        test al, 1
        jz .waitD
        ret
