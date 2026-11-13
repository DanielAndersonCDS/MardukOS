//! x86-64 architecture support.
mod exceptions;
mod gdt;
mod idt;

/// Install our GDT/TSS and IDT. Interrupts stay disabled (no IRQ controller yet).
pub fn init() {
    gdt::init();
    idt::init();
}
