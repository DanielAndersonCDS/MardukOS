; MardukOS boot entry. Limine enters here in 64-bit long mode, paging on,
; interrupts disabled, higher-half direct map active.
; Contract with Rust: extern "C" fn kmain() -> !   (no arguments yet)

bits 64
default rel

global _start
extern kmain

section .bss
align 16
stack_bottom:
    resb 16384              ; 16 KiB boot stack
stack_top:

section .text
_start:
    cli
    lea rsp, [stack_top]    ; 16-byte aligned
    xor ebp, ebp            ; terminate backtraces
    call kmain              ; must never return
.hang:
    cli
    hlt
    jmp .hang

section .note.GNU-stack noalloc noexec nowrite progbits
