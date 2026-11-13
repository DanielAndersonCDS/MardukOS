# MardukOS

Lightweight x86-64 OS (Assembly + Rust), UEFI + Limine, targeting Windows app
compatibility via WinCompat.

## Start in GitHub Codespaces

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/DanielAndersonCDS/MardukOS)

Open the repository in a browser-based Linux development environment. The first
setup automatically installs the toolchain, fetches Limine, builds the kernel,
and runs a QEMU boot smoke test. See [CODESPACES.md](CODESPACES.md).

## Local Debian/Ubuntu development

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

Exit QEMU with Ctrl-A, then X. Run `make test-boot` for a non-interactive boot
smoke test. Debug with `make debug`, then in another terminal:

```bash
gdb target/x86_64-unknown-none/debug/marduk-kernel -ex "target remote :1234"
```

**Phase 0 boot smoke test: PASS.** GitHub Actions builds the kernel, launches
UEFI + Limine in QEMU, and verifies the serial message above. This confirms the
initial boot path and UART output only; it does not mean the broader OS or Windows
application compatibility layers are implemented.
