//! Types related to task management
use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;
use crate::config::{kernel_stack_position, TRAP_CONTEXT};
use crate::mm::{MapPermission, MemorySet, PhysPageNum, VirtAddr, KERNEL_SPACE};
use crate::timer::get_time;
use crate::trap::{trap_handler, TrapContext};
use log::{debug, error};

/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub memory_set: MemorySet,
    pub trap_cx_ppn: PhysPageNum,
    pub base_size: usize,
    pub user_end: usize,   // 用户态结束时间
    pub kernel_end: usize, // 内核态结束时间
    pub time_start: usize, // 任务开始时间
    pub id: usize,
    pub call: [SyscallInfo; MAX_SYSCALL_NUM],
}

impl TaskControlBlock {
    pub(crate) fn show_end_time(&self) {
        debug!(
            "Task {} user end time: {},kernel end time {} | switch cost: {}",
            self.task_cx.get_app_id(),
            self.user_end,
            self.kernel_end,
            self.kernel_end - self.user_end
        );
    }
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

impl TaskControlBlock {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let task_status = TaskStatus::Ready;
        // map a kernel-stack in kernel space
        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(app_id);
        KERNEL_SPACE.exclusive_access().insert_framed_area(
            kernel_stack_bottom.into(),
            kernel_stack_top.into(),
            MapPermission::R | MapPermission::W,
        );
        let task_control_block = Self {
            task_status,
            task_cx: TaskContext::goto_trap_return(kernel_stack_top, app_id),
            memory_set,
            trap_cx_ppn,
            base_size: user_sp,
            user_end: 0,
            kernel_end: 0,
            time_start: 0,
            id: 0,
            call: [SyscallInfo { id: 0, times: 0 }; MAX_SYSCALL_NUM],
        };
        // prepare TrapContext in user space
        let trap_cx = task_control_block.get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().token(),
            kernel_stack_top,
            trap_handler as usize,
        );
        task_control_block
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    Ready,
    Running,
    Exited,
}
