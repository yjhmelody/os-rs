#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![allow(unused)]

global_asm!(include_str!("entry.asm"));
#[macro_use]
mod console;
mod interrupt;
mod panic;
mod sbi;

use sbi::sbi_call;

pub fn console_putchar(ch: u8) {
    sbi_call(1, ch as usize, 0, 0);
}

#[no_mangle]
extern "C" fn rust_main() {
    // init every modules
    interrupt::init();

    println!("Hello rCore-Tutorial!");

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
}
