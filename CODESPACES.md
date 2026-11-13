# Developing MardukOS in GitHub Codespaces

This repository includes a browser-based Linux development environment. You do not
need to install Linux, Rust, NASM, or QEMU on your Windows computer.

## Start the environment

1. Open the MardukOS repository on GitHub.
2. Select **Code → Codespaces → Create codespace on this branch**.
3. Wait for the first-time environment setup to finish.

The setup installs Rust, the `x86_64-unknown-none` target, NASM, QEMU, and OVMF
inside the Codespace.

## Build and boot

In the Codespaces terminal, run:

```bash
make fetch-limine
make run
```

QEMU runs in software-emulation mode and uses a serial console, so the virtual
machine appears in the terminal rather than opening a graphical window. Exit QEMU
with **Ctrl-A**, then **X**.

Expected serial output (once the kernel is successfully built and booted):

```
MardukOS kernel initialized.
```

## Important status

The current Phase 0 kernel is still an **untested scaffold**. A successful
Codespaces setup only confirms that the development tools are installed; it does
not guarantee that the kernel boots. If the build or boot fails, copy the complete
terminal error into an issue or a development conversation so it can be debugged.

QEMU software emulation is slower than hardware virtualization. It is used here
to avoid requiring nested virtualization support from the cloud environment.
