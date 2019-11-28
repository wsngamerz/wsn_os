read_disk:
    ; al => sectors to read
    ; cl => sector start
    
    pusha
    
    mov ah, 0x02 ; read disk flag
    mov ch, 0    ; cylinder
    mov dh, 0    ; head

    push bx
    mov bx, 0
    mov es, bx
    pop bx
    
    mov bx, 0x7c00 + 512

    int 0x13

    jc disk_error

    popa
    ret

    disk_error:
        mov si, STR_DISK_ERROR
        call printf
        
        jmp $
