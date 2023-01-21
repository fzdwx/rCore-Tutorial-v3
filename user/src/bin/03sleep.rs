#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::{get_time, task_info, yield_};

#[no_mangle]
fn main() -> i32 {
    let current_timer = get_time();
    let wait_for = current_timer + 3000;
    while get_time() < wait_for {
        yield_();
    }
    task_info(0).print();
    task_info(1).print();
    task_info(2).print();
    task_info(3).print();
    0
}
