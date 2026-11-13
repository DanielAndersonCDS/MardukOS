SHELL := /bin/bash

# Usage: make fetch-limine && make run
# Override OVMF paths if your distribution stores the firmware elsewhere.
OVMF      ?= $(firstword $(wildcard /usr/share/OVMF/OVMF_CODE_4M.fd /usr/share/OVMF/OVMF_CODE.fd /usr/share/edk2/ovmf/OVMF_CODE.fd /usr/share/qemu/OVMF.fd))
OVMF_VARS ?= $(firstword $(wildcard /usr/share/OVMF/OVMF_VARS_4M.fd /usr/share/OVMF/OVMF_VARS.fd /usr/share/edk2/ovmf/OVMF_VARS.fd /usr/share/qemu/OVMF_VARS.fd))
VARS_COPY := build/OVMF_VARS.fd
KERNEL    := target/x86_64-unknown-none/debug/marduk-kernel
ESP       := build/esp
LIMINE    := build/limine

.PHONY: all kernel esp run debug clean fetch-limine prepare-vars test-boot
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

# OVMF needs a writable variable-store copy alongside its read-only code image.
prepare-vars:
	test -n "$(OVMF_VARS)" || (echo "OVMF variable store not found; set OVMF_VARS=/path/to/OVMF_VARS.fd" && exit 1)
	test -f "$(VARS_COPY)" || (mkdir -p build && cp "$(OVMF_VARS)" "$(VARS_COPY)")

# Use the legacy PC machine so the directory-backed FAT disk is available as IDE.
# TCG is software emulation and does not require nested hardware virtualization.
QEMU_FLAGS = -machine pc,accel=tcg -m 256M -display none -serial stdio -no-reboot \
  -drive if=pflash,format=raw,readonly=on,file=$(OVMF) \
  -drive if=pflash,format=raw,file=$(VARS_COPY) \
  -drive format=raw,file=fat:rw:$(ESP)

run: esp prepare-vars
	test -n "$(OVMF)" || (echo "OVMF code image not found; set OVMF=/path/to/OVMF_CODE.fd" && exit 1)
	qemu-system-x86_64 $(QEMU_FLAGS)

# CI and Codespaces use this headless smoke test to verify the serial boot message.
test-boot: esp prepare-vars
	@set -e; \
	  rm -f build/test-boot.log; \
	  qemu-system-x86_64 $(QEMU_FLAGS) > build/test-boot.log 2>&1 & \
	  qemu_pid=$$!; \
	  for ((elapsed=0; elapsed<90; elapsed++)); do \
	    if grep -Fq "MardukOS kernel initialized." build/test-boot.log; then \
	      kill "$$qemu_pid" 2>/dev/null || true; \
	      wait "$$qemu_pid" 2>/dev/null || true; \
	      cat build/test-boot.log; \
	      echo "MardukOS boot smoke test passed."; \
	      exit 0; \
	    fi; \
	    if ! kill -0 "$$qemu_pid" 2>/dev/null; then break; fi; \
	    sleep 1; \
	  done; \
	  kill "$$qemu_pid" 2>/dev/null || true; \
	  wait "$$qemu_pid" 2>/dev/null || true; \
	  echo "ERROR: MardukOS did not print its expected kernel serial message."; \
	  cat build/test-boot.log; \
	  exit 1

debug: esp prepare-vars
	test -n "$(OVMF)" || (echo "OVMF code image not found; set OVMF=/path/to/OVMF_CODE.fd" && exit 1)
	qemu-system-x86_64 $(QEMU_FLAGS) -s -S

clean:
	cargo clean; rm -rf build
