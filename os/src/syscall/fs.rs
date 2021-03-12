const FD_STDOUT: usize = 1;
use crate::batch::is_space_available;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if !is_space_available(buf as usize, buf as usize + len) {
        // println!("[kernel] sys_write called on unavailable space: buf = {:#x}, len = {}", buf as usize, len);
        return -1;
    }
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        },
        _ => {
            // println!("Unsupported fd in sys_write!");
            -1
        }
    }
}