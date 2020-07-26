mod address;
mod allocator;
pub mod config;
pub mod frame;
mod heap;
mod range;

pub type MemoryResult<T> = Result<T, &'static str>;

pub fn init() {
    heap::init();

    unsafe { riscv::register::sstatus::set_sum() };

    println!("mod memory initialized");
}
