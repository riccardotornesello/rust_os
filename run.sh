# If OpenSBI binary is not present, download it
if [ ! -f opensbi-riscv32-generic-fw_dynamic.bin ]; then
    curl -LO https://github.com/qemu/qemu/raw/v8.0.4/pc-bios/opensbi-riscv32-generic-fw_dynamic.bin
fi

# Build the Rust OS for the RISC-V 32-bit target
cargo build --target riscv32i-unknown-none-elf

# Run the OS in QEMU with OpenSBI firmware
qemu-system-riscv32 -machine virt -bios default -nographic -serial mon:stdio --no-reboot -kernel target/riscv32i-unknown-none-elf/debug/rust_os
