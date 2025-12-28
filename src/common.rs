use crate::sbi;

pub enum Arg<'a> {
    Int(i32),
    UInt(u32),
    Str(&'a [u8]),
}

pub fn putchar(c: u8) {
    sbi::sbi_call(c as usize, 0, 0, 0, 0, 0, 0, 1);
}

pub fn printf(fmt: &[u8], args: &[Arg]) {
    let mut i = 0;
    let mut arg_index = 0;

    while i < fmt.len() {
        if fmt[i] == b'%' {
            i += 1;
            if i >= fmt.len() {
                break;
            }

            match fmt[i] {
                b's' => {
                    if let Some(Arg::Str(s)) = args.get(arg_index) {
                        for &c in *s {
                            putchar(c);
                        }
                    }
                }
                b'd' => {
                    if let Some(Arg::Int(n)) = args.get(arg_index) {
                        let mut number: i32 = *n;

                        if number < 0 {
                            putchar(b'-');
                            number = -number;
                        }

                        let mut divisor = 1;
                        while number / divisor >= 10 {
                            divisor *= 10;
                        }

                        while divisor > 0 {
                            let digit = (number / divisor) % 10;
                            putchar(b'0' + digit as u8);
                            divisor /= 10;
                        }
                    }
                }
                b'x' => {
                    if let Some(Arg::UInt(n)) = args.get(arg_index) {
                        let number: u32 = *n as u32;

                        putchar(b'0');
                        putchar(b'x');

                        for shift in (0..8).rev() {
                            let digit = (number >> (shift * 4)) & 0xF;
                            let c = if digit < 10 {
                                b'0' + digit as u8
                            } else {
                                b'a' + (digit as u8 - 10)
                            };
                            putchar(c);
                        }
                    }
                }
                _ => {}
            }

            arg_index += 1;
        } else {
            putchar(fmt[i]);
        }
        i += 1;
    }
}

#[macro_export]
macro_rules! PANIC {
    ($msg:expr) => {{
        $crate::common::printf(b"PANIC: %s\n", &[$crate::common::Arg::Str($msg)]);
        loop {
            unsafe {
                core::arch::asm!("wfi");
            }
        }
    }};
}
