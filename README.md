# MardukOS

Lightweight x86-64 OS (Assembly + Rust), UEFI + Limine, targeting Windows app
compatibility via WinCompat.

## Phase 0 development

### GitHub Codespaces (recommended)

You can build and test MardukOS in a browser-based Linux environment without
installing the toolchain on your computer. See [CODESPACES.md](CODESPACES.md).

### Debian/Ubuntu (local)

```bash
sudo apt install nasm qemu-system-x86 ovmf git make build-essential
curl https://sh.rustup.rs -sSf | sh
make fetch-limine
make run
```

Expected serial output after a successful boot:

```
MardukOS kernel initialized.
```

Exit QEMU with Ctrl-A, then X. Debug with `make debug`, then in another terminal:

```bash
gdb target/x86_64-unknown-none/debug/marduk-kernel -ex "target remote :1234"
```

**Status: UNTESTED scaffold.** The Codespaces configuration installs the
development tools; the kernel still needs its first successful build and boot.
