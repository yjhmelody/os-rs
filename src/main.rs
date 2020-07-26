#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![allow(unused)]

global_asm!(include_str!("entry.asm"));
#[macro_use]
mod console;
mod interrupt;
mod memory;
mod panic;
mod sbi;

extern crate alloc;
use sbi::sbi_call;

pub fn console_putchar(ch: u8) {
    sbi_call(1, ch as usize, 0, 0);
}

#[no_mangle]
extern "C" fn rust_main() {
    // init every modules
    interrupt::init();
    memory::init();

    use alloc::boxed::Box;
    use alloc::vec::Vec;

    let v = Box::new(5);
    assert_eq!(*v, 5);
    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10000);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }
    println!("heap test passed");
    println!("Hello rCore-Tutorial!");
    println!("{}", *memory::config::KERNEL_END_ADDRESS);

    // 物理页分配
    for _ in 0..2 {
        let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err),
        };
        let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err),
        };
        println!("{} and {}", frame_0.address(), frame_1.address());
    }

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
}
