//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_GET_TASKINFO: usize = 1 << 10;
pub const SYSCALL_STAT: usize = (1 << 10) + 1;

mod fs;
mod process;
mod stat;

use fs::*;
use process::*;
use stat::*;

use crate::batch::SYSCALL_COUNTER;

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    let mut syscall_counter = SYSCALL_COUNTER.exclusive_access();
    syscall_counter.count(syscall_id);
    drop(syscall_counter);

    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_GET_TASKINFO => sys_get_taskinfo(),
        SYSCALL_STAT => sys_stat(),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
