#![no_std]

extern crate alloc;

mod block_dev;
mod layout;
mod efs;
mod bitmap;
mod vfs;
mod block_cache;

pub const BLOCK_SZ: usize = 512;
pub use block_dev::BlockDevice;
pub use efs::EasyFileSystem;
pub use vfs::Inode;
pub use layout::NAME_LENGTH_LIMIT;
use layout::*;
use bitmap::Bitmap;
use block_cache::get_block_cache;