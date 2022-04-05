.intel_syntax noprefix

.section .text
.globl _start

_start:
    mov rbx, 10
    mov rax, 1
    int 0x80
