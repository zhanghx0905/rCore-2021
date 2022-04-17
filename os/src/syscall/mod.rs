const SYSCALL_DUP: usize = 24;
const SYSCALL_OPEN: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;

const SYSCALL_SPAWN: usize = 400;
const SYSCALL_MUNMAP: usize = 215;
const SYSCALL_MMAP: usize = 222;
const SYSCALL_SET_PRIORITY: usize = 140;
const SYSCALL_MAIL_READ: usize = 401;
const SYSCALL_MAIL_WRITE: usize = 402;
const SYSCALL_UNLINKAT: usize = 35;
const SYSCALL_LINKAT: usize = 37;
const SYSCALL_FSTAT: usize = 80;

mod fs;
mod process;

use fs::*;
use process::*;
use crate::fs::Stat;

pub fn syscall(syscall_id: usize, args: [usize; 5]) -> isize {
    match syscall_id {
        SYSCALL_DUP=> sys_dup(args[0]),
        SYSCALL_OPEN => sys_open(args[1] as *const u8, args[2] as u32),
        SYSCALL_CLOSE => sys_close(args[0]),
        SYSCALL_PIPE => sys_pipe(args[0] as *mut usize),
        SYSCALL_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_FORK => sys_fork(),
        SYSCALL_EXEC => sys_exec(args[0] as *const u8, args[1] as *const usize),
        SYSCALL_WAITPID => sys_waitpid(args[0] as isize, args[1] as *mut i32),

        SYSCALL_SET_PRIORITY => sys_set_priority(args[0] as isize),
        SYSCALL_MMAP => sys_mmap(args[0], args[1], args[2]),
        SYSCALL_MUNMAP => sys_munmap(args[0], args[1]),
        SYSCALL_MAIL_READ => sys_mail_read(args[0] as *const u8, args[1]),
        SYSCALL_SPAWN => sys_spawn(args[0] as *const u8, args[1] as *const usize),
        SYSCALL_MAIL_WRITE => sys_mail_write(args[0], args[1] as *mut u8, args[2]),
        SYSCALL_FSTAT => sys_fstat(args[0], args[1] as *mut Stat),
        SYSCALL_LINKAT => sys_linkat(args[1], args[3]),
        SYSCALL_UNLINKAT => sys_unlinkat(args[1]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
