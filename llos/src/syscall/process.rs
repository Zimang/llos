use crate::config::{MAX_APP_NUM,MAX_SYSCALL_NUM};
use crate::task::{exit_current_and_run_next,suspend_current_and_run_next,TaskStatus};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]

pub struct TimeVal{
    pub sec:usize,
    pub usec:usize,
}

pub struct TaskInfo{
    status:TaskStatus,
    syscall_times:[u32;MAX_SYSCALL_NUM],
    time:usize,
}

//退出任务并返回，退出码
pub fn sys_exit(exit_code:i32)->!{
   info!("[kernel] 应用退出，返回码为{}",exit_code);
   exit_current_and_run_next();
   panic!("sys_exit 无法获取");
}

//停止当前任务并运行下一个任务
pub fn sys_yield()->isize{
    suspend_current_and_run_next();
    0
}

//获取时间（秒和微秒）
pub fn sys_get_time(ts:*mut TimeVal,_tz:usize)->iszie{
    let us=get_time_us();
    unsafe{
        *ts=TimeVal{
            sec:us/1_000_000,
            usec:us%1_000_000,
        }
    }
    0
}

pub fn sys_task_info(ti:*mut TaskInfo)->isize{
    -1
}


