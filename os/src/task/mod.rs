mod context;
mod switch;
mod task;
mod manager;
mod processor;
mod pid;

use crate::fs::{open_file, OpenFlags};
use crate::mm::{MapPermission, VirtAddr};
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};
use alloc::sync::Arc;
use manager::{fetch_task, get_ready_task_by_pid};
use lazy_static::*;
pub use context::TaskContext;

pub use processor::{
    run_tasks,
    current_task,
    current_user_token,
    current_trap_cx,
    take_current_task,
    schedule,
};
pub use manager::add_task;
pub use pid::{PidHandle, pid_alloc, KernelStack};

pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- hold current PCB lock
    let mut task_inner = task.acquire_inner_lock();
    let task_cx_ptr2 = task_inner.get_task_cx_ptr2();
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- release current PCB lock

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr2);
}

pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();
    // **** hold current PCB lock
    let mut inner = task.acquire_inner_lock();
    // Change status to Zombie
    inner.task_status = TaskStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;
    // do not move to its parent but under initproc

    // ++++++ hold initproc PCB lock here
    {
        let mut initproc_inner = INITPROC.acquire_inner_lock();
        for child in inner.children.iter() {
            child.acquire_inner_lock().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ release parent PCB lock here

    inner.children.clear();
    // deallocate user space
    inner.memory_set.recycle_data_pages();
    drop(inner);
    // **** release current PCB lock
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let _unused: usize = 0;
    schedule(&_unused as *const _);
}

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new({
        let inode = open_file("initproc", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice())
    });
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}


pub fn get_task_by_pid(pid: usize) ->  Option<Arc<TaskControlBlock>> {
    let current = current_task().unwrap();
    if current.pid.0 == pid {
        return Some(current);
    }
    get_ready_task_by_pid(pid)
}

pub fn insert_framed_area(
    start_va: VirtAddr,
    end_va: VirtAddr,
    permission: MapPermission,
) -> Result<(), ()> {
    current_task()
        .unwrap()
        .acquire_inner_lock()
        .memory_set
        .insert_framed_area(start_va, end_va, permission)?;
    Ok(())
}

pub fn remove_framed_area(start_va: VirtAddr, end_va: VirtAddr) -> Result<(), ()> {
    current_task()
        .unwrap()
        .acquire_inner_lock()
        .memory_set
        .remove_framed_area(start_va, end_va)?;
    Ok(())
}
