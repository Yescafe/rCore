use crate::batch::SYSCALL_COUNTER;

pub fn sys_stat() -> isize {
    let syscall_counter = SYSCALL_COUNTER.exclusive_access();
    for i in 0..4 {
        let syscall_name: &'static str;
        match i {
            0 => syscall_name = "SYSCALL_WRITE",
            1 => syscall_name = "SYSCALL_EXIT",
            2 => syscall_name = "SYSCALL_GET_TASKINFO",
            3 => syscall_name = "SYSCALL_STAT",
            _ => panic!("Never reach"),
        }
        println!("{:>25}  {:<5}", syscall_name, syscall_counter.c[i]);
    }
    drop(syscall_counter);
    0
}
