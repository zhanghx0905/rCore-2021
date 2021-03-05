use crate::mm::{MapPermission, VirtAddr};
use crate::task::{
    exit_current_and_run_next, insert_framed_area, remove_framed_area, suspend_current_and_run_next,
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

pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    if (port & !0x7) != 0 || (port & 0x7) == 0 || start % 4096 != 0 {
        return -1;
    }
    let mut permission = MapPermission::U;
    if port & 0x1 != 0 {
        permission |= MapPermission::R;
    }
    if port & 0x2 != 0 {
        permission |= MapPermission::W;
    }
    if port & 0x4 != 0 {
        permission |= MapPermission::X;
    }
    let end = VirtAddr::from(VirtAddr(start + len).ceil());
    match insert_framed_area(start.into(), end, permission) {
        Ok(_) => (end.0 - start) as isize,
        Err(_) => -1,
    }
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    match remove_framed_area(start.into(), VirtAddr(start + len).ceil().into()) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
