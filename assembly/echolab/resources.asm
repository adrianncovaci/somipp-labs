; printing resources from inside bx register
print:
    pusha

printloop:
    mov al, [bx]
    cmp al, 0
    je doneprinting
    mov ah, 0x0e
    int 0x10

    add bx, 1
    jmp printloop

doneprinting:
    popa
    ret

; printing out a new line
println:
    pusha
    mov ah, 0x0e
    mov al, 0x0a
    int 0x10
    mov al, 0x0d
    int 0x10
    popa
    ret

get_string:
    pusha
    xor cl, cl

    loop:
        mov ah, 0
        int 0x16 ; wait for keys to be typed

        cmp al, 0x08 ; handle backspace
        je backspace

        cmp al, 0x0d ; handle enter
        je enter

        cmp cl, 0x0ef ; in order to get 256 available characters, this number should be == 0x100 == 
        je loop       ; but qemu doesn't let me use more than 239 (0x0ef)
    
        mov ah, 0x0e ; print the character
        int 0x10

        inc cl
        jmp loop

backspace:
    cmp cl, 0 ; if first char of the line
    je loop

    dec di
    mov byte[di], 0 ; delete char
    dec cl          ; decrement count register
    mov ah, 0x0e
    mov al, 0x08
    int 10h

    mov al, ''
    int 10h
    mov al, 0x08    ; printing the backspace first time, moves the cursor one position to the left
    int 10h         ; printing it twice, clears the character
    jmp loop

enter:
    mov al, 0
    stosb
    ;mov ah, 0x0e
    ;mov al, 0x0d
    ;int 0x10
    ;mov al, 0x0a
    ;int 0x10
    call println
    popa
    ret
