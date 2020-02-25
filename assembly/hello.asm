BITS 16

start:
	mov ax, 07C0h		; 4k stack space
	add ax, 288		; (4096 + 512) / 16 bytes per paragraph
	mov ss, ax
	mov sp, 4096

	mov ax, 07C0h		; Set data segment to where we're loaded from <4>
	mov ds, ax


    mov ah, 0x0e

    ;; PRINTING CHARACTER BY CHARACTER SECTION
    mov al, 'H'
    int 0x10
    mov al, 'e'
    int 0x10
    mov al, 'l'
    int 0x10
    mov al, 'l'
    int 0x10
    mov al, 'o'
    int 0x10

    ;; END CHAR BY CHAR SECTION

    mov si, text_string
    call print_string
    jmp $
    
    text_string db 13,10,'Welcome to my OS!', 0

print_string:
    mov ah, 0Eh

.repeat:
    lodsb
    cmp al, 0
    je .finished
    int 0x10
    jmp .repeat


    
.finished:
    ret

    times 510-($-$$) db 0
    dw 0xaa55
