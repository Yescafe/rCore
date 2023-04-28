//! Types related to task management

use super::{TaskContext, TASK_MANAGER};

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub kern_time: usize,
    pub user_time: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

pub fn enter_user_mode() {
    TASK_MANAGER.enter_user_mode()
}

pub fn escape_user_mode() {
    TASK_MANAGER.escape_user_mode()
}
