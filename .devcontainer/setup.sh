#!/usr/bin/env bash
set -euo pipefail

echo "==> Installing MardukOS system dependencies"
sudo apt-get update
sudo DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
  ca-certificates curl git make nasm qemu-system-x86 ovmf build-essential
sudo rm -rf /var/lib/apt/lists/*

echo "==> Installing the minimal Rust toolchain"
if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
fi
export PATH="$HOME/.cargo/bin:$PATH"
rustup toolchain install stable --profile minimal
rustup target add x86_64-unknown-none

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

echo "==> Fetching Limine"
make fetch-limine

echo "==> Building MardukOS and running the first boot smoke test"
make test-boot

echo "==> MardukOS Codespaces environment is ready"
rustc --version
cargo --version
nasm -v
qemu-system-x86_64 --version | head -n 1
