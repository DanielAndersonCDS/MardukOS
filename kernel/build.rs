// Assembles every NASM source with nasm and passes the objects plus the linker script to the linker.
use std::{env, path::PathBuf, process::Command};

const ASM_SOURCES: &[&str] = &["boot/entry.asm", "kernel/src/arch/x86_64/isr.asm"];

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("..");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    for src in ASM_SOURCES {
        let asm = root.join(src);
        let stem = asm.file_stem().unwrap().to_string_lossy().into_owned();
        let obj = out.join(format!("{stem}.o"));

        let status = Command::new("nasm")
            .args(["-f", "elf64", "-g", "-F", "dwarf"])
            .arg(&asm)
            .arg("-o")
            .arg(&obj)
            .status()
            .expect("failed to run nasm (is it installed?)");
        assert!(status.success(), "nasm failed on {src}");

        println!("cargo:rustc-link-arg={}", obj.display());
        println!("cargo:rerun-if-changed={}", asm.display());
    }

    let ld = root.join("kernel/linker.ld");
    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rustc-link-arg=-zmax-page-size=0x1000");
    println!("cargo:rerun-if-changed={}", ld.display());
}
