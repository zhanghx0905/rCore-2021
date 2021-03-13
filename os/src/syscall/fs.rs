use crate::mm::{
    UserBuffer,
    translated_byte_buffer,
    translated_refmut,
    translated_str,
};
use crate::task::{current_user_token, current_task, get_task_by_pid};
use crate::fs::{make_pipe, OpenFlags, open_file, Stat, create_hard_link, remove_hard_link};
use alloc::sync::Arc;


pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release Task lock manually to avoid deadlock
        drop(inner);
        match translated_byte_buffer(token, buf, len) {
            Some(buffer) => file.write(UserBuffer::new(buffer)) as isize,
            None => -1,
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
        if !file.readable() {
            return -1;
        }
        // release Task lock manually to avoid deadlock
        drop(inner);
        match translated_byte_buffer(token, buf, len) {
            Some(buffer) => file.read(UserBuffer::new(buffer)) as isize,
            None => -1,
        }
    } else {
        -1
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    println!("path: {} flags: {}", path, flags);
    if let Some(inode) = open_file(
        path.as_str(),
        OpenFlags::from_bits(flags).unwrap()
    ) {
        let mut inner = task.acquire_inner_lock();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
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

pub fn sys_dup(fd: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    let new_fd = inner.alloc_fd();
    inner.fd_table[new_fd] = Some(Arc::clone(inner.fd_table[fd].as_ref().unwrap()));
    new_fd as isize
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

pub fn sys_fstat(fd: usize, st: *mut Stat) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    match inner.fd_table[fd].as_ref().unwrap().get_stat() {
        Some(stat) => {
            *translated_refmut(token, st) =stat;
        },
        None => {return -1;},
    };
    0
}

pub fn sys_linkat(old_name: usize, new_name: usize) -> isize {
    let token = current_user_token();
    let old = translated_str(token, old_name as *const u8);
    let new = translated_str(token, new_name as *const u8);
    create_hard_link(old.as_str(), new.as_str())
}

pub fn sys_unlinkat(name: usize) -> isize {
    let token = current_user_token();
    let name = translated_str(token, name as *const u8);
    remove_hard_link(name.as_str())
}