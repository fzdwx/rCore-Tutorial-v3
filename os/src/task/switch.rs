//! Rust wrapper around `__switch`.
//!
//! Switching to a different task's context happens here. The actual
//! implementation must not be in Rust and (essentially) has to be in assembly
//! language (Do you know why?), so this module really is just a wrapper around
//! `switch.S`.

use super::TaskContext;
use core::arch::global_asm;
use log::debug;

global_asm!(include_str!("switch.S"));

extern "C" {
    /// Switch to the context of `next_task_cx_ptr`, saving the current context
    /// in `current_task_cx_ptr`.
    fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}

/// switch 交换两个 task,替换执行流
pub fn switch__(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext) {
    unsafe {
        let current = current_task_cx_ptr.as_ref().unwrap();
        let next = next_task_cx_ptr.as_ref().unwrap();
        debug!(
            "switch from {:?} to {:?}",
            current.get_app_id(),
            next.get_app_id(),
        );
        __switch(current_task_cx_ptr, next_task_cx_ptr);
    }
}
