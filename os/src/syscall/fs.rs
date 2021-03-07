use crate::fs::make_pipe;
use crate::mm::{translated_byte_buffer, translated_refmut, UserBuffer};
use crate::task::{current_task, current_user_token, get_task_by_pid};

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        // release Task lock manually to avoid deadlock
        drop(inner);
        if let Some(buffer) = translated_byte_buffer(token, buf, len) {
            file.write(UserBuffer::new(buffer)) as isize
        } else {
            -1
        }
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        // release Task lock manually to avoid deadlock
        drop(inner);
        if let Some(buffer) = translated_byte_buffer(token, buf, len) {
            file.read(UserBuffer::new(buffer)) as isize
        } else {
            -1
        }
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

pub fn sys_pipe(pipe: *mut usize) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let mut inner = task.acquire_inner_lock();
    let (pipe_read, pipe_write) = make_pipe();
    let read_fd = inner.alloc_fd();
    inner.fd_table[read_fd] = Some(pipe_read);
    let write_fd = inner.alloc_fd();
    inner.fd_table[write_fd] = Some(pipe_write);
    *translated_refmut(token, pipe) = read_fd;
    *translated_refmut(token, unsafe { pipe.add(1) }) = write_fd;
    0
}

pub fn sys_mail_read(buf: *const u8, len: usize) -> isize {
    let token = current_user_token(); // 先获取 token, 防止死锁
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if !inner.mails.readable() {
        return -1;
    } else if len == 0 {
        return 0;
    }
    let len = len.min(256);
    if let Some(buffer) = translated_byte_buffer(token, buf, len) {
        inner.mails.read(UserBuffer::new(buffer)) as isize
    } else {
        -1
    }
}

pub fn sys_mail_write(pid: usize, buf: *mut u8, len: usize) -> isize {
    let token = current_user_token();
    let task = match get_task_by_pid(pid) {
        Some(p) => p,
        None => return -1,
    };
    let mut inner = task.acquire_inner_lock();
    if !inner.mails.writable() {
        return -1;
    } else if len == 0 {
        return 0;
    }
    let len = len.min(256);
    if let Some(buffer) = translated_byte_buffer(token, buf, len) {
        inner.mails.write(UserBuffer::new(buffer)) as isize
    } else {
        -1
    }
}
