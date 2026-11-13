# Usage: make fetch-limine && make run
# Override OVMF if needed: make run OVMF=/path/to/OVMF_CODE.fd
OVMF    ?= $(firstword $(wildcard /usr/share/OVMF/OVMF_CODE_4M.fd /usr/share/OVMF/OVMF_CODE.fd /usr/share/edk2/ovmf/OVMF_CODE.fd /usr/share/qemu/OVMF.fd))
KERNEL  := target/x86_64-unknown-none/debug/marduk-kernel
ESP     := build/esp
LIMINE  := build/limine

.PHONY: all kernel esp run debug clean fetch-limine
all: esp

kernel:
	cargo build

fetch-limine:
	mkdir -p build
	test -d $(LIMINE) || git clone --depth=1 --branch=v8.x-binary https://github.com/limine-bootloader/limine.git $(LIMINE)

esp: kernel
	test -f $(LIMINE)/BOOTX64.EFI || (echo "Run 'make fetch-limine' first" && exit 1)
	mkdir -p $(ESP)/EFI/BOOT $(ESP)/boot
	cp $(LIMINE)/BOOTX64.EFI $(ESP)/EFI/BOOT/BOOTX64.EFI
	cp tools/limine.conf $(ESP)/EFI/BOOT/limine.conf
	cp $(KERNEL) $(ESP)/boot/marduk-kernel

# TCG uses software emulation, so nested hardware virtualization is not required.
QEMU_FLAGS = -machine q35,accel=tcg -m 256M -display none -serial stdio -no-reboot \
  -drive if=pflash,format=raw,readonly=on,file=$(OVMF) \
  -drive format=raw,file=fat:rw:$(ESP)

run: esp
	test -n "$(OVMF)" || (echo "OVMF not found; set OVMF=/path/to/OVMF_CODE.fd" && exit 1)
	qemu-system-x86_64 $(QEMU_FLAGS)

debug: esp
	test -n "$(OVMF)" || (echo "OVMF not found; set OVMF=/path/to/OVMF_CODE.fd" && exit 1)
	qemu-system-x86_64 $(QEMU_FLAGS) -s -S

clean:
	cargo clean; rm -rf build
