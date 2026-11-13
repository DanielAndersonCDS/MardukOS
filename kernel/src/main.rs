#![no_std]
#![no_main]

mod arch;
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

    arch::init();
    serial::write_str("GDT, TSS and IDT loaded.\n");

    selftest();

    serial::write_str("Boot sequence complete; halting.\n");
    halt()
}

/// Deliberate faults, enabled one at a time with cargo features (see Cargo.toml).
fn selftest() {
    #[cfg(feature = "test-breakpoint")]
    {
        // SAFETY: int3 is handled by our #BP handler, which returns.
        unsafe { core::arch::asm!("int3", options(nostack)) };
        serial::write_str("Breakpoint handled, execution resumed.\n");
    }
    #[cfg(feature = "test-divide")]
    {
        // SAFETY: intentionally divides by zero to raise #DE; handler halts.
        unsafe {
            core::arch::asm!("div {z:e}", z = in(reg) 0u32,
                inout("eax") 1u32 => _, inout("edx") 0u32 => _, options(nostack));
        }
    }
    #[cfg(feature = "test-pagefault")]
    {
        // SAFETY: intentionally reads an unmapped address to raise #PF; handler halts.
        unsafe { core::ptr::read_volatile(0xdead_b000usize as *const u8) };
    }
    #[cfg(feature = "test-doublefault")]
    {
        // SAFETY: intentionally points RSP at a non-canonical address, then traps. Delivering
        // the exception faults, the follow-up fault faults too, and the CPU raises #DF on its IST stack.
        unsafe {
            core::arch::asm!(
                "mov rsp, {bad}",
                "int3",
                bad = in(reg) 0x8000_0000_0000_0000u64,
                options(noreturn)
            );
        }
    }
}

pub fn halt() -> ! {
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
