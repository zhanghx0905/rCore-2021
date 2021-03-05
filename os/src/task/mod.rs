mod context;
mod scheduler;
mod switch;
mod task;

use crate::config::{BIG_STRIDE, MAX_APP_NUM, TIME_LIMIT};
use crate::loader::{get_num_app, init_app_cx};
use alloc::collections::binary_heap::BinaryHeap;
use core::cell::RefCell;
use lazy_static::*;
use scheduler::Stride;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

pub struct TaskManager {
    // num_app: usize,
    inner: RefCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
    current_stride: usize,
    heap: BinaryHeap<Stride>,
    tasks_time_counter: [usize; MAX_APP_NUM],
    tasks_priority: [usize; MAX_APP_NUM],
}

unsafe impl Sync for TaskManager {}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx_ptr: 0,
            task_status: TaskStatus::UnInit
        }; MAX_APP_NUM];
        let mut heap = BinaryHeap::new();
        for i in 0..num_app {
            tasks[i].task_cx_ptr = init_app_cx(i) as *const _ as usize;
            tasks[i].task_status = TaskStatus::Ready;
            heap.push(Stride { stride: 0, pid: i });
        }
        TaskManager {
            // num_app,
            inner: RefCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
                current_stride: 0,
                heap: heap,
                tasks_time_counter: [0; MAX_APP_NUM],
                tasks_priority: [16; MAX_APP_NUM],
            }),
        }
    };
}

impl TaskManager {
    fn run_first_task(&self) {
        let mut inner = self.inner.borrow_mut();
        let pid = inner.heap.pop().unwrap().pid;
        inner.current_task = pid;
        inner.tasks[pid].task_status = TaskStatus::Running;
        let next_task_cx_ptr2 = inner.tasks[pid].get_task_cx_ptr2();
        drop(inner);
        let _unused: usize = 0;
        unsafe {
            __switch(&_unused as *const _, next_task_cx_ptr2);
        }
    }

    fn mark_current_suspended(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks_time_counter[current] += 1;
        inner.tasks[current].task_status = if inner.tasks_time_counter[current] >= TIME_LIMIT {
            TaskStatus::Exited
        } else {
            TaskStatus::Ready
        };
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    fn find_next_task(&self) -> Option<Stride> {
        self.inner.borrow_mut().heap.pop()
        // let current = inner.current_task;
        // (current + 1..current + self.num_app + 1)
        //     .map(|id| id % self.num_app)
        //     .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    fn update_heap(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        if inner.tasks[current].task_status == TaskStatus::Ready {
            let prio = inner.tasks_priority[current];
            let current_stride = inner.current_stride;
            inner.heap.push(Stride {
                stride: current_stride + BIG_STRIDE / prio,
                pid: current,
            });
        }
    }
    fn run_next_task(&self) {
        self.update_heap();
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.borrow_mut();
            let current = inner.current_task;
            inner.current_stride = next.stride;
            let next = next.pid;

            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr2 = inner.tasks[current].get_task_cx_ptr2();
            let next_task_cx_ptr2 = inner.tasks[next].get_task_cx_ptr2();
            core::mem::drop(inner);
            unsafe {
                __switch(current_task_cx_ptr2, next_task_cx_ptr2);
            }
        } else {
            panic!("All applications completed!");
        }
    }

    fn set_priority(&self, prio: usize) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks_priority[current] = prio;
    }
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn set_priority(prio: usize) {
    TASK_MANAGER.set_priority(prio);
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn get_current_appid() -> usize {
    TASK_MANAGER.inner.borrow().current_task
}