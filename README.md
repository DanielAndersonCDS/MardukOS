# MardukOS
Lightweight x86-64 OS (Assembly + Rust), UEFI + Limine, targeting Windows app compatibility via WinCompat.

## Phase 0 build & boot (Debian/Ubuntu)
    sudo apt install nasm qemu-system-x86 ovmf git make build-essential
    curl https://sh.rustup.rs -sSf | sh
    make fetch-limine
    make run
Expected serial output: `MardukOS kernel initialized.`  Exit QEMU with Ctrl-A X.
Debug: `make debug` then `gdb target/x86_64-unknown-none/debug/marduk-kernel -ex "target remote :1234"`.
Status: UNTESTED scaffold.
