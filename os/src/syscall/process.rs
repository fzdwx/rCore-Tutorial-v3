//! Process management syscalls
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TASK_MANAGER};
use crate::timer::{get_time, get_time_ms};
use log::info;

#[repr(C)]
#[derive(Debug, Default)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time in milliseconds
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_mark_user_end_time() -> isize {
    TASK_MANAGER.mark_user_end();
    0
}
