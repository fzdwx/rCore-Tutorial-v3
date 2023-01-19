//! Implementation of [`TaskContext`]

/// Task Context
#[derive(Copy, Clone)]
#[repr(C)]
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

    /// set task context {__restore ASM funciton, kernel stack, s_0..12 }
    pub fn goto_restore(kstack_ptr: usize, app_id: i64) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
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
