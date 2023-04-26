//! App management syscalls
use crate::batch::{run_next_app, APP_MANAGER};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}

pub fn sys_get_taskinfo() -> isize {
    let app_manager = APP_MANAGER.exclusive_access();
    let task_id = app_manager.get_current_app() - 1;
    drop(app_manager);
    println!("task id: {}", task_id);
    task_id as isize
}
