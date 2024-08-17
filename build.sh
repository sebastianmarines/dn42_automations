#!/bin/zsh

# Check the target architecture
if [[ "$1" == "aarch64" ]]; then
    echo "Building for aarch64-unknown-linux-gnu..."
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc cargo build --target aarch64-unknown-linux-gnu
elif [[ "$1" == "x86_64" ]]; then
    echo "Building for x86_64-unknown-linux-gnu..."
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc cargo build --target=x86_64-unknown-linux-gnu
else
    echo "Unknown target architecture. Please specify either 'aarch64' or 'x86_64'."
    exit 1
fi
