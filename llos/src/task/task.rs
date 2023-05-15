use super::TaskContext;

#[derive(Copy,CLone)]

pub struct TaskCOntrolBlock{
    pub task_status: TaskStatus,
    pub task_cx:TaskContext,
}

#[derive(Copy,CLone,PartialEq)]
pub enum TaskStatus{
    Uninit,
    Ready,
    Running,
    Exited,
}
