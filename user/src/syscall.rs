use core::arch::asm;

const SYSCALL_WRITE: usize = 1;
const SYSCALL_EXIT: usize = 2;
const SYSCALL_YIELD: usize = 3;
const SYSCALL_GET_TIME: usize = 4;
const SYSCALL_TASK_INFO: usize = 5;
pub const MAX_SYSCALL_NUM: usize = 6;

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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

impl TaskInfo {
    pub fn new() -> Self {
        Self {
            id: 0,
            status: TaskStatus::UnInit,
            call: [SyscallInfo::new(); MAX_SYSCALL_NUM],
            time: 0,
        }
    }
    pub fn print(&self) {
        println!("Task {}:", self.id);
        println!("  Status: {:?}", self.status);
        println!("  Time: {} ms", self.time);
        println!("  Syscall:");
        for i in 0..MAX_SYSCALL_NUM {
            if self.call[i].times != 0 {
                println!("    {}: {} times", self.call[i].id, self.call[i].times);
            }
        }
    }
}

impl SyscallInfo {
    pub fn new() -> Self {
        Self { id: 0, times: 0 }
    }
}

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}

pub fn sys_task_info(id: usize, ts: *mut TaskInfo) -> isize {
    syscall(SYSCALL_TASK_INFO, [id, ts as usize, 0])
}
