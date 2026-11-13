# Developing MardukOS in GitHub Codespaces

[**Create or resume the MardukOS Codespace**](https://codespaces.new/DanielAndersonCDS/MardukOS)

You can develop from your browser without installing Linux, Rust, NASM, or QEMU
on your Windows computer.

## First-time setup

When GitHub creates the Codespace, its automatic post-create script:

1. Installs Rust and the `x86_64-unknown-none` target.
2. Installs NASM, QEMU, OVMF, Git, and Make.
3. Downloads the Limine bootloader.
4. Builds the kernel and runs a QEMU boot smoke test.

The setup is considered successful only when the kernel prints:

```
MardukOS kernel initialized.
```

The first setup can take a few minutes because it installs the development tools
and boots the virtual machine once.

## Run MardukOS again

After setup finishes, open the Codespaces terminal and run:

```bash
make run
```

QEMU uses software emulation and a serial console, so its output appears in the
terminal rather than a separate graphical window. Exit QEMU with **Ctrl-A**, then
**X**.

To repeat the non-interactive boot check at any time:

```bash
make test-boot
```

## What is verified

GitHub Actions has successfully built and booted the current Phase 0 kernel on
Ubuntu 24.04 with UEFI, Limine, and QEMU. The smoke test checks the kernel's
initial entry point and COM1 serial output. It does not yet test memory
management, scheduling, storage, drivers, or Windows application compatibility.

QEMU uses TCG software emulation, so nested hardware virtualization is not
required. Emulation is slower than hardware virtualization.
