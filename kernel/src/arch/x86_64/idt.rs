//! IDT with the 32 CPU exception vectors. Stubs live in isr.asm.
use super::gdt::{DOUBLE_FAULT_IST, KERNEL_CS};
use core::arch::asm;
use core::mem::size_of;
use core::ptr::addr_of_mut;

#[repr(C)]
#[derive(Clone, Copy)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    zero: u32,
}

impl IdtEntry {
    const MISSING: Self = Self { offset_low: 0, selector: 0, ist: 0, type_attr: 0, offset_mid: 0, offset_high: 0, zero: 0 };

    fn interrupt_gate(handler: u64, ist: u8) -> Self {
        Self {
            offset_low: handler as u16,
            selector: KERNEL_CS,
            ist,
            type_attr: 0x8E, // present, DPL0, 64-bit interrupt gate (clears IF)
            offset_mid: (handler >> 16) as u16,
            offset_high: (handler >> 32) as u32,
            zero: 0,
        }
    }
}

#[repr(C, packed)]
struct TablePointer {
    limit: u16,
    base: u64,
}

static mut IDT: [IdtEntry; 256] = [IdtEntry::MISSING; 256];

extern "C" {
    /// Defined in isr.asm: addresses of the 32 exception stubs.
    static isr_stub_table: [u64; 32];
}

pub fn init() {
    // SAFETY: single-threaded early boot; the IDT static lives forever and the stub
    // addresses come from our own assembly.
    unsafe {
        let idt = addr_of_mut!(IDT) as *mut IdtEntry;
        for i in 0..32 {
            let ist = if i == 8 { DOUBLE_FAULT_IST } else { 0 };
            idt.add(i).write(IdtEntry::interrupt_gate(isr_stub_table[i], ist));
        }
        let idtr = TablePointer {
            limit: (256 * size_of::<IdtEntry>() - 1) as u16,
            base: idt as u64,
        };
        asm!("lidt [{}]", in(reg) &idtr, options(readonly, nostack, preserves_flags));
    }
}
