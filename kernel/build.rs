// Assembles boot/entry.asm with NASM and passes it, plus the linker script, to the linker.
use std::{env, path::PathBuf, process::Command};

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("..");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let asm = root.join("boot/entry.asm");
    let obj = out.join("entry.o");

    let status = Command::new("nasm")
        .args(["-f", "elf64", "-g", "-F", "dwarf"])
        .arg(&asm)
        .arg("-o")
        .arg(&obj)
        .status()
        .expect("failed to run nasm (is it installed?)");
    assert!(status.success(), "nasm failed");

    println!("cargo:rustc-link-arg={}", obj.display());
    println!("cargo:rustc-link-arg=-T{}", root.join("kernel/linker.ld").display());
    println!("cargo:rustc-link-arg=-zmax-page-size=0x1000");
    println!("cargo:rerun-if-changed={}", asm.display());
    println!("cargo:rerun-if-changed={}", root.join("kernel/linker.ld").display());
}
