; Exception entry stubs for vectors 0-31.
; Builds a uniform frame (see TrapFrame in exceptions.rs), calls
; exception_handler(frame: *mut TrapFrame) per the System V ABI, then irets.
;
; Stack alignment: the CPU aligns RSP to 16 bytes before pushing its frame in
; 64-bit mode. We then push 5 (CPU) + 1 (error/dummy) + 1 (vector) + 15 GPRs
; = 22 qwords = 176 bytes, so RSP is 16-byte aligned at the call.

bits 64
default rel

global isr_stub_table
extern exception_handler

%macro ISR_NOERR 1
isr_stub_%1:
    push 0              ; dummy error code
    push %1
    jmp isr_common
%endmacro

%macro ISR_ERR 1
isr_stub_%1:            ; CPU already pushed the error code
    push %1
    jmp isr_common
%endmacro

section .text

ISR_NOERR 0
ISR_NOERR 1
ISR_NOERR 2
ISR_NOERR 3
ISR_NOERR 4
ISR_NOERR 5
ISR_NOERR 6
ISR_NOERR 7
ISR_ERR   8
ISR_NOERR 9
ISR_ERR   10
ISR_ERR   11
ISR_ERR   12
ISR_ERR   13
ISR_ERR   14
ISR_NOERR 15
ISR_NOERR 16
ISR_ERR   17
ISR_NOERR 18
ISR_NOERR 19
ISR_NOERR 20
ISR_ERR   21
ISR_NOERR 22
ISR_NOERR 23
ISR_NOERR 24
ISR_NOERR 25
ISR_NOERR 26
ISR_NOERR 27
ISR_NOERR 28
ISR_ERR   29
ISR_ERR   30
ISR_NOERR 31

isr_common:
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15
    mov rdi, rsp
    cld
    call exception_handler
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    add rsp, 16         ; drop vector + error code
    iretq

section .rodata
align 8
isr_stub_table:
%assign i 0
%rep 32
    dq isr_stub_ %+ i
%assign i i+1
%endrep

section .note.GNU-stack noalloc noexec nowrite progbits
