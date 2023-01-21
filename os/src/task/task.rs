//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;
use crate::timer::get_time;
use log::error;

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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskInfo {
    pub id: usize,
    pub status: TaskStatus,
    pub call: [SyscallInfo; MAX_SYSCALL_NUM],
    pub time: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SyscallInfo {
    pub id: usize,
    pub times: usize,
}

impl TaskInfo {
    pub fn print(&self) {
        error!("Task {}:", self.id);
        error!("  Status: {:?}", self.status);
        error!("  Time: {} ms", self.time);
        error!("  Syscall:");
        for i in 0..MAX_SYSCALL_NUM {
            if self.call[i].times != 0 {
                error!("    {}: {} times", self.call[i].id, self.call[i].times);
            }
        }
    }
}

impl TaskControlBlock {
    pub fn to_task_info(&self) -> TaskInfo {
        let time = match self.task_status {
            TaskStatus::Exited => self.user_end - self.time_start,
            _ => get_time() - self.time_start,
        };

        let info = TaskInfo {
            id: self.id,
            status: self.task_status,
            call: self.call,
            time,
        };
        info
    }
}
