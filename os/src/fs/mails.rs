use crate::mm::UserBuffer;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

const MAX_MAILS_NUM: usize = 16;

#[derive(Debug, Clone)]
pub struct Mails(pub VecDeque<Vec<u8>>);

impl Mails {
    pub fn readable(&self) -> bool {
        self.0.len() > 0
    }
    pub fn writable(&self) -> bool {
        self.0.len() < MAX_MAILS_NUM
    }
    pub fn read(&mut self, buf: UserBuffer) -> usize {
        let mut buf_iter = buf.into_iter();
        let mut read_size = 0usize;
        let mail = self.0.pop_front().unwrap();
        for c in mail {
            if let Some(byte_ref) = buf_iter.next() {
                unsafe { *byte_ref = c; }
                read_size += 1;
            } else {
                break;
            }
        }
        read_size
    }
    pub fn write(&mut self, buf: UserBuffer) -> usize {
        let mut write_size = 0usize;
        let mut mail: Vec<u8> = Vec::new();
        for c in buf {
            unsafe{ mail.push(*c); }
            write_size += 1;
        }
        self.0.push_back(mail);
        write_size
    }
}