//! 中断模块
//!
//!

mod timer;
mod context;
mod handler;

/// 初始化中断相关的子模块
///
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    handler::init();
    timer::init();
    
    println!("mod interrupt initialized");
}
