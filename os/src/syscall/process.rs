use crate::task::{
    suspend_current_and_run_next,
    exit_current_and_run_next,
    set_priority,
};
use crate::timer::get_time_ms;

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_set_priority(prio: isize) -> isize {
    if prio < 2 {
        -1
    } else {
        set_priority(prio as usize);
        prio
    }
}