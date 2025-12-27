#![no_std]
#![no_main]

mod common;
mod sbi;

use core::arch::asm;
use core::panic::PanicInfo;

unsafe extern "C" {
    static __stack_top: u8;
}


#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() {
    let fmt = b"Hello, %s %d %x!\n";
    let name = b"Rust OS";
    let version = 1;
    let random_hex: u32 = 0xDEADBEEF;
    common::printf(fmt, &[common::Arg::Str(name), common::Arg::Int(version), common::Arg::UInt(random_hex)]);

    PANIC!(b"Kernel panic occurred!");

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
pub fn boot() -> ! {
    unsafe {
        asm!(
            "mv sp, {stack_top}",
            "j {kernel_main}",
            stack_top = in(reg) { &__stack_top },
            kernel_main = sym kernel_main,
            options(noreturn)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
