mod inode;
mod mails;
mod pipe;
mod stdio;

use crate::mm::UserBuffer;

pub trait File: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
    fn get_stat(&self) -> Option<Stat> {
        None
    }
}

pub use inode::{
    create_hard_link, list_apps, open_file, remove_hard_link, OSInode, OpenFlags, Stat,
};
pub use mails::Mails;
pub use pipe::{make_pipe, Pipe};
pub use stdio::{Stdin, Stdout};
