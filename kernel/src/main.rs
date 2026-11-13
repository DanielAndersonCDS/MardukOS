#![no_std]
#![no_main]

mod serial;

/// Limine base revision tag (protocol revision 3). Limine zeroes the third
/// word if it supports the requested revision. Must be kept by the linker.
#[used]
#[link_section = ".limine_requests"]
static BASE_REVISION: [u64; 3] = [0xf956_2b2d_5c95_a6c8, 0x6a7b_3849_4453_6bdc, 3];

/// Called from boot/entry.asm. Never returns.
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    serial::init();
    serial::write_str("MardukOS kernel initialized.\n");
    halt()
}

fn halt() -> ! {
    loop {
        // SAFETY: halting the CPU with interrupts disabled has no memory effects.
        unsafe { core::arch::asm!("cli; hlt", options(nomem, nostack)) };
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    let _ = writeln!(serial::Com1, "KERNEL PANIC: {}", info);
    halt()
}
