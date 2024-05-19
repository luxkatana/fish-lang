global _start
_start:
    mov rax, 14
    push rax
    PUSH QWORD [rsp + 0]
    pop rax
    push rax; y = x
    mov rax, 60
    mov rdi, 0
    syscall
