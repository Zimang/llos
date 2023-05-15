const SYSCALL_WRITE:usize=64;
const SYSCALL_EXIT:usize=93;
const SYSCALL_YIELD:usize=124;
const SYSCALL_GET_TIME:usize=169;
const SYSCALL_TASK_INFO:usize=410;


mod fs;
mod process;

use fs::*;
use process::*;

pub fn syscall(syscall_id:usize,args:[usize;3])->isize{
    match syscall_id{
        SYSCALL_WRITE=>sys_write(args[0],arg[1] as *const u8,args[2]),
        SYSCALL_EXIT=>sys_exit(args[0] as i32),
        SYSCALL_YIELD=>sys_yied(),
        //get_time需要什么参数呢？
        SYSCALL_GET_TIME=>sys_get_time(args[0] as *mut TimeVal,args[1]),
        //sys_task_info是干吗的呢？
        SYSCALL_TASK_INFO=>sys_task_info(args[0] as *mut TaskInfo),    
        _=>panic!("未支持系统调用 id为{}",syscall_id),
    }
}
