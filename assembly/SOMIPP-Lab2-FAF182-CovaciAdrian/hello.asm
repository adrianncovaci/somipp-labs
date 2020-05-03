org 0x7c00

jmp begin

    greeting    db  'Hello, World',0
    red         db  'Sunt rosu', 0
    grn         db  'Sunt verde', 0
    blu         db  'Sunt albastru', 0

begin:
    mov al, 25
    mov bh, 71h
    mov ch, 0
    mov cl, 0
    mov dh, 25h
    mov dl, 80
    mov ah, 07h
    int 10h

    mov dh, 2
    mov dl, 10
    mov ah, 2
    mov bh, 0
    int 10h
    lea si,[greeting]


    mov ah, 09h
    mov bl, 02h
    mov cx, 3
    int 10h
    mov al, 03

print:
    mov al, byte[si]
    mov ah, 0eh
    int 10h
    inc si
    cmp al, 0
    jz end_first_string
    jmp print

end_first_string:

    mov ah, 06h
    mov cl, 00h
    mov dl, 18h
    mov ch, 7h
    mov dh, 10h
    mov bh, 29h
    int 10h

    mov ah, 06h
    mov cl, 18h
    mov dl, 36h
    mov ch, 7h
    mov dh, 10h
    mov bh, 47h
    int 10h

    mov ah, 06h
    mov cl, 36h
    mov dl, 4fh
    mov ch, 7h
    mov dh, 10h
    mov bh, 14h
    int 10h

    mov ah, 4ch
    int 21h


    mov ah, 2
    mov bh, 0
    mov dl, 24h,
    mov dh, 0ch
    int 10h

    lea si,[red]

print_second:
    mov al, byte[si]
    mov ah, 0eh
    int 10h
    inc si
    cmp al, 0
    jz end_second_string
    jmp print_second

end_second_string:
    mov ah, 2
    mov bh, 0
    mov dl, 5h,
    mov dh, 0ch
    int 10h

    lea si,[grn]

print_green:
    mov al, byte[si]
    mov ah, 0eh
    int 10h
    inc si
    cmp al, 0
    jz end_third_string
    jmp print_green

end_third_string:

    mov ah, 2
    mov bh, 0
    mov dl, 40h,
    mov dh, 0ch
    int 10h

    lea si,[blu]

print_blue:
    mov al, byte[si]
    mov ah, 0eh
    int 10h
    inc si
    cmp al, 0
    jz end_fourth_string
    jmp print_blue

end_fourth_string:
    mov ah, 01h
    mov cx, 0007h
    int 10h
    mov ah, 0Ah
    mov al, 03
    int 10h

    mov ah, 0Ch
    mov al, 23h
    mov cx, 1
    mov dx, 1
    int 10h
    hlt
