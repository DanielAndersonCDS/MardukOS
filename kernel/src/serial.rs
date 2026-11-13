//! Minimal 16550 UART driver for COM1 (0x3F8). Polled, output only.
use core::arch::asm;
use core::fmt;

const COM1: u16 = 0x3F8;

/// SAFETY: caller must ensure `port` is a valid I/O port to write.
unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

/// SAFETY: caller must ensure `port` is a valid I/O port to read.
unsafe fn inb(port: u16) -> u8 {
    let v: u8;
    asm!("in al, dx", out("al") v, in("dx") port, options(nomem, nostack, preserves_flags));
    v
}

pub fn init() {
    // SAFETY: standard COM1 register programming; ports are fixed on PC hardware/QEMU.
    unsafe {
        outb(COM1 + 1, 0x00); // disable interrupts
        outb(COM1 + 3, 0x80); // DLAB on
        outb(COM1, 0x03);     // divisor 3 -> 38400 baud
        outb(COM1 + 1, 0x00);
        outb(COM1 + 3, 0x03); // 8N1, DLAB off
        outb(COM1 + 2, 0xC7); // enable FIFO
        outb(COM1 + 4, 0x03); // DTR + RTS
    }
}

pub fn write_byte(b: u8) {
    // SAFETY: reading LSR and writing THR on COM1.
    unsafe {
        while inb(COM1 + 5) & 0x20 == 0 {}
        outb(COM1, b);
    }
}

pub fn write_str(s: &str) {
    for b in s.bytes() {
        if b == b'\n' {
            write_byte(b'\r');
        }
        write_byte(b);
    }
}

pub struct Com1;
impl fmt::Write for Com1 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_str(s);
        Ok(())
    }
}
