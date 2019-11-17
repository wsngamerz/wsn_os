testA20:
    pusha

    mov ax, [0x7dfe]

    push bx
    mov bx, 0xffff
    mov es, bx
    pop bx

    mov bx, 0x7e0e
    mov dx, [es:bx]

    cmp ax, dx
    je .continue

    popa
    mov ax, 1
    ret

    .continue:
        mov ax, [0x7dff]

        push bx
        mov bx, 0xffff
        mov es, bx
        pop bx

        mov bx, 0x7e0f

        mov dx, [es:bx]

        cmp ax, dx
        je .exit
            
        popa
        mov ax, 1
        ret
    
    .exit:
        popa
        xor ax, ax
        ret

