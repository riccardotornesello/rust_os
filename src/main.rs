#![no_std]
#![no_main]

mod common;
mod memory;
mod sbi;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() {
    memory::init_memory();

    // Boot message
    let fmt = b"Hello, %s %d!\nStack top address: %x\n";
    let name = b"Rust OS";
    let version: i32 = 1;
    common::printf(
        fmt,
        &[
            common::Arg::Str(name),
            common::Arg::Int(version),
            common::Arg::UInt(unsafe { &memory::__stack_top as *const usize as u32 }),
        ],
    );

    // Test memory
    let paddr0 = memory::alloc_pages(2);
    let paddr1 = memory::alloc_pages(1);
    common::printf(
        b"alloc_pages test: paddr0=%x\n",
        &[common::Arg::UInt(paddr0 as u32)],
    );
    common::printf(
        b"alloc_pages test: paddr1=%x\n",
        &[common::Arg::UInt(paddr1 as u32)],
    );

    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
pub fn boot() -> ! {
    unsafe {
        core::arch::asm!(
            "mv sp, {stack_top}",
            "j {kernel_main}",
            stack_top = in(reg) { &memory::__stack_top },
            kernel_main = sym kernel_main,
            options(noreturn)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    PANIC!(b"Kernel panic occurred!");
}
