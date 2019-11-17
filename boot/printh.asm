printh:
    ; value of dx is the hex val
    push cx
    push di
    push bx
    
    mov si, HEX_PATTERN
    mov cl, 12
    mov di, 2

    hexLoop:
        mov bx, dx                 ; copy dx to bx to preserve val
        shr bx, cl                 ; shift by 12 bits (3 bytes) right
        and bx, 0x000f             ; mask first 3 digits
        mov bx, [bx + HEX_TABLE]   ; load ascii val of hex into bx
        mov [HEX_PATTERN + di], bl ; insert into correct spot
        sub cl, 4                  ; change bits shifted in next itter
        inc di                     ; ++ insertion location for next itter

        cmp di, 6
        je finish   ; exit loop if done

        jmp hexLoop ; continue loop if not done

    finish:
        call printf ; print now built HEX_PATTERN string
        pop bx      ; return values
        pop di
        pop cx
        ret         ; return to prev pos


HEX_PATTERN: db "0x****", 0x0a, 0x0d, 0
HEX_TABLE: db "0123456789abcdef"
