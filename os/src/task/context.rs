//! Implementation of [`TaskContext`]
use crate::trap::trap_return;

#[repr(C)]
/// task context structure containing some registers
pub struct TaskContext {
    /// return address ( e.g. __restore ) of __switch ASM function
    ra: usize,
    /// kernel stack pointer of app
    sp: usize,
    /// callee saved registers:  s 0..11
    s: [usize; 12],
    /// app id
    app_id: i64,
}

impl TaskContext {
    /// init task context
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
            app_id: -1,
        }
    }
    /// set Task Context{__restore ASM funciton: trap_return, sp: kstack_ptr, s: s_0..12}
    pub fn goto_trap_return(kstack_ptr: usize, app_id: usize) -> Self {
        let app_id = app_id as i64;
        Self {
            ra: trap_return as usize,
            sp: kstack_ptr,
            s: [0; 12],
            app_id,
        }
    }

    /// get current app id
    pub fn get_app_id(&self) -> i64 {
        self.app_id
    }
}
