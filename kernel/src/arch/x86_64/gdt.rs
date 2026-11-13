//! GDT and TSS. Layout: 0x00 null, 0x08 kernel code, 0x10 kernel data, 0x18 TSS (16 bytes).
//! User segments are added in Phase 2 (their order is constrained by SYSRET).
use core::arch::asm;
use core::mem::size_of;
use core::ptr::addr_of_mut;

pub const KERNEL_CS: u16 = 0x08;
pub const KERNEL_DS: u16 = 0x10;
const TSS_SEL: u16 = 0x18;
/// IST index (1-based, as used in IDT entries) holding the double-fault stack.
pub const DOUBLE_FAULT_IST: u8 = 1;

/// 64-bit TSS (104 bytes). packed(4) matches the hardware layout (u64s at 4-byte offsets).
#[repr(C, packed(4))]
struct Tss {
    _reserved0: u32,
    rsp: [u64; 3],
    _reserved1: u64,
    ist: [u64; 7],
    _reserved2: u64,
    _reserved3: u16,
    iomap_base: u16,
}

#[repr(C, align(16))]
struct Stack([u8; 16 * 1024]);

static mut DOUBLE_FAULT_STACK: Stack = Stack([0; 16 * 1024]);
static mut TSS: Tss = Tss {
    _reserved0: 0,
    rsp: [0; 3],
    _reserved1: 0,
    ist: [0; 7],
    _reserved2: 0,
    _reserved3: 0,
    iomap_base: 0,
};
static mut GDT: [u64; 5] = [0; 5];

#[repr(C, packed)]
struct TablePointer {
    limit: u16,
    base: u64,
}

pub fn init() {
    // SAFETY: single-threaded early boot; statics are only touched here, and the
    // descriptors/pointers we build describe memory that lives for the whole kernel run.
    unsafe {
        let tss = addr_of_mut!(TSS);
        let stack_top = addr_of_mut!(DOUBLE_FAULT_STACK) as u64 + size_of::<Stack>() as u64;
        (*tss).ist[0] = stack_top;
        (*tss).iomap_base = size_of::<Tss>() as u16; // beyond limit => no I/O bitmap

        let base = tss as u64;
        let limit = (size_of::<Tss>() - 1) as u64;
        let tss_low = (limit & 0xFFFF)
            | ((base & 0xFF_FFFF) << 16)
            | (0x89u64 << 40) // present, DPL0, type = available 64-bit TSS
            | (((limit >> 16) & 0xF) << 48)
            | (((base >> 24) & 0xFF) << 56);
        let tss_high = base >> 32;

        *addr_of_mut!(GDT) = [
            0,
            0x00AF_9A00_0000_FFFF, // kernel code: L=1, P, DPL0, exec/read
            0x00CF_9200_0000_FFFF, // kernel data: P, DPL0, read/write
            tss_low,
            tss_high,
        ];

        let gdtr = TablePointer {
            limit: (size_of::<[u64; 5]>() - 1) as u16,
            base: addr_of_mut!(GDT) as u64,
        };
        asm!("lgdt [{}]", in(reg) &gdtr, options(readonly, nostack, preserves_flags));

        // Reload CS with a far return, then the data segments.
        asm!(
            "push {sel}",
            "lea {tmp}, [rip + 2f]",
            "push {tmp}",
            "retfq",
            "2:",
            sel = in(reg) KERNEL_CS as u64,
            tmp = out(reg) _,
            options(preserves_flags),
        );
        asm!(
            "mov ds, {0:x}",
            "mov es, {0:x}",
            "mov ss, {0:x}",
            in(reg) KERNEL_DS,
            options(nostack, preserves_flags),
        );
        asm!("ltr {0:x}", in(reg) TSS_SEL, options(nostack, preserves_flags));
    }
}
