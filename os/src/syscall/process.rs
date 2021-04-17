use crate::task::{
    suspend_current_and_run_next,
    exit_current_and_run_next,
    current_task,
    current_user_token,
    add_task,
    insert_framed_area,
    remove_framed_area,
};
use crate::timer::get_time_us;
use crate::mm::{
    translated_str,
    translated_refmut,
};
use crate::mm::{MapPermission, VirtAddr};
use crate::loader::get_app_data_by_name;
use alloc::sync::Arc;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    let ptr = translated_refmut(current_user_token(), ts);
    *ptr = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    0
}

pub fn sys_set_priority(prio: isize) -> isize {
    if prio < 2 {
        -1
    } else {
        // set_priority(prio as usize);
        prio
    }
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
    let end = VirtAddr::from(VirtAddr(start + len).ceil());
    match remove_framed_area(start.into(), end) {
        Ok(_) => (end.0 - start) as isize,
        Err(_) => -1,
    }
}

pub fn sys_getpid() -> isize {
    current_task().unwrap().pid.0 as isize
}

pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.acquire_inner_lock().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    add_task(new_task);
    new_pid as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
}

pub fn sys_spawn(path: *const u8) -> isize {
    let path = translated_str(current_user_token(), path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let new_task = current_task().unwrap().fork();
        new_task.exec(data);
        let new_pid = new_task.pid.0;
        add_task(new_task);
        new_pid as isize
    } else {
        -1
    }
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = current_task().unwrap();
    // find a child process

    // ---- hold current PCB lock
    let mut inner = task.acquire_inner_lock();
    if inner.children
        .iter()
        .find(|p| {pid == -1 || pid as usize == p.getpid()})
        .is_none() {
        return -1;
        // ---- release current PCB lock
    }
    let pair = inner.children
        .iter()
        .enumerate()
        .find(|(_, p)| {
            // ++++ temporarily hold child PCB lock
            p.acquire_inner_lock().is_zombie() && (pid == -1 || pid as usize == p.getpid())
            // ++++ release child PCB lock
        });
    if let Some((idx, _)) = pair {
        let child = inner.children.remove(idx);
        // confirm that child will be deallocated after removing from children list
        assert_eq!(Arc::strong_count(&child), 1);
        let found_pid = child.getpid();
        // ++++ temporarily hold child lock
        let exit_code = child.acquire_inner_lock().exit_code;
        // ++++ release child PCB lock
        *translated_refmut(inner.memory_set.token(), exit_code_ptr) = exit_code;
        found_pid as isize
    } else {
        -2
    }
    // ---- release current PCB lock automatically
}