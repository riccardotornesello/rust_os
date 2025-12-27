#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

unsafe extern "C" {
    static mut __bss: u8;
    static mut __bss_end: u8;
    static __stack_top: u8;
}

pub struct SbiRet {
    pub error: usize,
    pub value: usize,
}

pub fn sbi_call(
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    fid: usize,
    eid: usize,
) -> SbiRet {
    let mut error;
    let mut value;

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
            options(nostack)
        );
    }

    return SbiRet { error, value };
}

fn putchar(c: u8) {
    sbi_call(c as usize, 0, 0, 0, 0, 0, 0, 1);
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() {
    let message = b"Hello, Rust OS!\n";
    for &c in message {
        putchar(c);
    }

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
