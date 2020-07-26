use crate::sbi::set_timer;
use riscv::register::{sie, sstatus, time};

pub static mut TICKS: usize = 0;

pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            println!("{} tick", TICKS);
        }
    }
}

fn set_next_timeout() {
    static INTERVAL: usize = 100000;
    set_timer(time::read() + INTERVAL)
}

pub fn init() {
    unsafe {
        // open stie
        sie::set_stimer();

        sstatus::set_sie();
    }

    set_next_timeout();
}
