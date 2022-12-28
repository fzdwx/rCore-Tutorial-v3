#![no_std]
#![no_main]

use core::arch::global_asm;
use crate::sbi::{console_putchar, putchar, shutdown};

mod lang_items;
mod sbi;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    putchar('O');
    putchar('K');
    putchar('\n');
    shutdown();
}

#[no_mangle]
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe {
            (a as *mut u8).write_volatile(0)
        }
    })
}