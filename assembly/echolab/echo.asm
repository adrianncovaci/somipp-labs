[org 0x7c00]

mov bx, msg
call print
call println

mainloop:
    mov bx, prompt
    call print
    
    call get_string
    jmp mainloop


jmp $
%include "resources.asm"

    


buffer times 64 db 0
prompt: db 'os$> ', 0
msg: db 'Hello, World', 0
times 510-($-$$) db 0
dw 0xaa55
