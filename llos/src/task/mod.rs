//! 实现task的管理
//! 包括启动任务，转换任务
//!
//! 一个全局任务管理器实例TASK_MANAGER管理操作系统的所有任务
//!
//! 注意__switch,该函数的实现可能并非你所想的，他是通过汇编封装实现的

mod context;
mod switch;
#[allow(clippy::module_incepton)]
mod task;

use crate::config::{MAX_APP_NUM,MAX_SYSCALL_NUM};
use crate::loader::{get_nume_app,init_app_cx};
use crate::sync::UPSafeCell;
use lazy_static::*;
pub use switch::__switch;
pub use task::{TaskControlBlock,TaskStatus};
pub use context::TaskContext;

pub struct TaskManager{
    num_app:uszie,
    //使用inner值来获取mutable access
    inner:UPSafeCell<TaskManagaerInner>,
}

struct TaskManagerInner{
    //任务列表
    tasks:[TaskControlBlock;MAX_APP_NUM],
    //当前任务id
    current_task:usize,
}



lazy_static!{
    //通过lazy_staic实现的TaskManger实例化、
    pub static ref TASK_MANGER:TaskManager={
        let num_app=get_num_app();
        let mut tasks=[TaskCOntrolBlock{
            task_cx:TaskContext::zero_init(),
            task_status:TaskStatus::Uninit,
        };MAX_APP_NUM];
        for (i,t) in tasks.iter_mut().enumrate().take(num_app){
            t.task_cx=TaskContext::goto_retore(init_app_cx(i));
            t.task_status=TaskStatus::Ready;
        } 
        TaskManager{
            num_app,
            inner:unsafe{
                UPSafeCell::new(TaskManagerInner{
                    tasks,
                    current_task:0,
                })
            }
        }
    }
}

impl TaskManager{
    //运行在任务列表中的第一个任务
    //
    //通常来说第一个任务是一个空转的任务，但是我们这里运行的是一个实际的任务
    fn run_first_task(&self)->!{
        let mut inner=self.inner.exclusive_access();
        let task0=&mut inner.tasks[0];
        task0.task_status=TaskStatus::Running;
        let next_task_cx_ptr=&task0.task_cx as *const TaskContext;
        //显示归还借用
        drop(inner);
        let mut _unused=TaskContext::zero_init();
        unsafe{
            __switch(&mut _unused as *mut TaskContext,next_task_cx_ptr);
        }
        panic!(“run_first_task 无法获取”);
    }

    //转变当前running为ready
    fn mark_current_suspended(&self){
        let mut inner=self.inner.exclusive_access();
        let current=inner.current_task;
        inner.tasks[current].task_satus=TaskStatus::Exited;
    }
    
    //exit
    fn mark_current_exited(&self){
        let mut inner=self.inner.exclusive_access();
        let current=inner.current_task;
        inner.tasks[current].task_status=TaskStatus::Exited;
    }

    //找到下一个任务，返回任务id
    //此时是顺序查找
    fn find_next_task(&self)->Option<usize>{
        let inner=self.inner.exclusive_access();
        let current=inner.current_task;
        (current+1..current+self.num_app+1)
            .map(|id|id%self.num_app)
            .find(|id| inner.tasks[*id].task_stauts==TaskStauts::Ready)
    }
    fn run_next_task(&self){
        if let Some(next)=self.find_next_task(){
            let mut inner=self.inner.exclusive_access();
            let current=inner.current_task;
            inner.task[next].task_status=TaskStatus::Running;
            inner.current_task=next;
            let curretn_task_cx_ptr=&mut inner.tasks[current].task.cx as *mut TaskContext;
            let next_task_cx_ptr =&inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            
            unsafe{
                __switch(current_task_cx_ptr,next_task_cx_ptr)
            }
        }else{
            panic!("完成所有任务");
        }
    }
}

pub fn run_first_task(){
    TASK_MANAGER.run_first_task();
}

fn run_next_task(){
    TASK_MANAGER.run_next_task();
}

fn mark_current_suspended(){
    TASK_MANAGER.mark_current_suspended();
}

pub fn suspend_current_and_run_next(){
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next(){
    mark_current_exited();
    run_next_task();
}
