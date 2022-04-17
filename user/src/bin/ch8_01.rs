#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::fork;

const NUM: usize = 10;

#[no_mangle]
pub fn main() -> i32 {
    for _ in 0..NUM {
        let pid = fork();
        if pid != 0 {
            println!("forked new process {}", pid);
        }
    }
    0
}
