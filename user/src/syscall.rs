use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_GET_TASKINFO: usize = 1 << 10;
const SYSCALL_STAT: usize = (1 << 10) + 1;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

pub fn sys_get_taskinfo() -> isize {
    syscall(SYSCALL_GET_TASKINFO, [0, 0, 0])
}

pub fn sys_stat() -> isize {
    syscall(SYSCALL_STAT, [0, 0, 0])
}
