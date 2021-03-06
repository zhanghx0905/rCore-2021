use crate::mm::translated_byte_buffer;
use crate::task::{current_user_token, suspend_current_and_run_next};
use crate::sbi::console_getchar;

const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            if let Some(buffers) = translated_byte_buffer(current_user_token(), buf, len) {
                for buffer in buffers {
                    print!("{}", core::str::from_utf8(buffer).unwrap());
                }
                len as isize
            } else {
                -1
            }
        }
        _ => {
            println!("Unsupported fd in sys_write!");
            -1
        }
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDIN => {
            assert_eq!(len, 1, "Only support len = 1 in sys_read!");
            let mut c: usize;
            loop {
                c = console_getchar();
                if c == 0 {
                    suspend_current_and_run_next();
                    continue;
                } else {
                    break;
                }
            }
            let ch = c as u8;
            let mut buffers = translated_byte_buffer(current_user_token(), buf, len).unwrap();
            unsafe { buffers[0].as_mut_ptr().write_volatile(ch); }
            1
        }
        _ => {
            panic!("Unsupported fd in sys_read!");
        }
    }
}