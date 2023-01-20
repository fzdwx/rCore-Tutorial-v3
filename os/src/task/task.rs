//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub user_end: usize,   // 用户态结束时间
    pub kernel_end: usize, // 内核态结束时间
    pub time_start: usize, // 任务开始时间
    pub id: usize,
    pub call: [SyscallInfo; MAX_SYSCALL_NUM],
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

#[derive(Copy, Clone)]
pub struct TaskInfo {
    id: usize,
    status: TaskStatus,
    call: [SyscallInfo; MAX_SYSCALL_NUM],
    time: usize,
}

#[derive(Copy, Clone)]
pub struct SyscallInfo {
    pub id: usize,
    times: usize,
}

impl TaskControlBlock {
    pub fn to_task_info(&self) -> TaskInfo {
        TaskInfo {
            id: self.id,
            status: self.task_status,
            call: self.call,
            time: self.kernel_end - self.time_start,
        }
    }
}
