//! Rust side of exception handling. Called from isr_common in isr.asm.
use crate::serial::Com1;
use core::arch::asm;
use core::fmt::Write;

/// Must match the push order in isr.asm exactly (lowest address first).
#[repr(C)]
pub struct TrapFrame {
    r15: u64, r14: u64, r13: u64, r12: u64, r11: u64, r10: u64, r9: u64, r8: u64,
    rbp: u64, rdi: u64, rsi: u64, rdx: u64, rcx: u64, rbx: u64, rax: u64,
    vector: u64,
    error: u64,
    // Pushed by the CPU:
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

const NAMES: [&str; 32] = [
    "Divide Error (#DE)", "Debug (#DB)", "Non-Maskable Interrupt", "Breakpoint (#BP)",
    "Overflow (#OF)", "Bound Range Exceeded (#BR)", "Invalid Opcode (#UD)", "Device Not Available (#NM)",
    "Double Fault (#DF)", "Coprocessor Segment Overrun", "Invalid TSS (#TS)", "Segment Not Present (#NP)",
    "Stack-Segment Fault (#SS)", "General Protection Fault (#GP)", "Page Fault (#PF)", "Reserved",
    "x87 Floating-Point (#MF)", "Alignment Check (#AC)", "Machine Check (#MC)", "SIMD Floating-Point (#XM)",
    "Virtualization (#VE)", "Control Protection (#CP)", "Reserved", "Reserved",
    "Reserved", "Reserved", "Reserved", "Reserved",
    "Hypervisor Injection (#HV)", "VMM Communication (#VC)", "Security (#SX)", "Reserved",
];

#[no_mangle]
pub extern "C" fn exception_handler(f: &mut TrapFrame) {
    let mut out = Com1;
    let v = f.vector as usize;

    if v == 3 {
        let _ = writeln!(out, "[exception] Breakpoint at RIP={:#018x}, resuming", f.rip);
        return;
    }

    let _ = writeln!(out, "\n*** CPU EXCEPTION: {} (vector {}) ***", NAMES.get(v).unwrap_or(&"?"), v);
    let _ = writeln!(out, "error={:#x}  RIP={:#018x}  CS={:#x}  RFLAGS={:#x}", f.error, f.rip, f.cs, f.rflags);
    // In an IST exception, the CPU saves the interrupted RSP in the frame,
    // but the handler is executing on the TSS-provided IST stack. Report the
    // live handler RSP for #DF and retain the interrupted value separately.
    let handler_rsp: u64;
    // SAFETY: reading RSP does not modify machine state.
    unsafe { asm!("mov {}, rsp", out(reg) handler_rsp, options(nomem, nostack, preserves_flags)) };
    let reported_rsp = if v == 8 { handler_rsp } else { f.rsp };
    let _ = writeln!(out, "RSP={:#018x}  SS={:#x}", reported_rsp, f.ss);
    if v == 8 {
        let _ = writeln!(out, "Interrupted RSP={:#018x}", f.rsp);
    }
    let _ = writeln!(out, "RAX={:#018x} RBX={:#018x} RCX={:#018x} RDX={:#018x}", f.rax, f.rbx, f.rcx, f.rdx);
    let _ = writeln!(out, "RSI={:#018x} RDI={:#018x} RBP={:#018x}", f.rsi, f.rdi, f.rbp);

    if v == 14 {
        let cr2: u64;
        // SAFETY: reading CR2 has no side effects.
        unsafe { asm!("mov {}, cr2", out(reg) cr2, options(nomem, nostack, preserves_flags)) };
        let e = f.error;
        let _ = writeln!(
            out,
            "CR2={:#018x}  [{} {} {}{}{}]",
            cr2,
            if e & 1 != 0 { "protection-violation" } else { "not-present" },
            if e & 2 != 0 { "write" } else { "read" },
            if e & 4 != 0 { "user" } else { "supervisor" },
            if e & 8 != 0 { " reserved-bit" } else { "" },
            if e & 16 != 0 { " instruction-fetch" } else { "" },
        );
    }

    let _ = writeln!(out, "System halted.");
    crate::halt()
}
