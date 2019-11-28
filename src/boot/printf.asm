; output string to screen
printf:
    pusha
    ; loop through the si register one char at a time and output the char
    str_loop:
        mov al, [si]
        cmp al, 0
        jne print_char
        popa
        ret
    
    ; 'method' to output the char
    print_char:
        mov ah, 0x0e
        int 0x10
        inc si
        jmp str_loop
