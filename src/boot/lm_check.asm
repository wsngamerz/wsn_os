lm_check:
    pusha

    pushfd
    pop eax
    mov ecx, eax

    xor eax, 1 << 21
    
    push eax
    popfd

    pushfd
    pop eax

    xor eax, ecx
    jz .done

    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb .done

    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz .done

    mov si, STR_LM_COMPAT
    call printf
    popa
    ret

    .done:
        popa
        mov si, STR_NO_LM
        call printf
        jmp $ 