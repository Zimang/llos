core::arch::global_asm!(include_str!("Switch.S"));

use super::TaskContext;

extern "C"{
    pub fn __switch(current_task_cx_ptr:*mut TaskContext,
                    next_task_cx_ptr:*const TaskContext);
}
