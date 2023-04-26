#![no_std]
#![no_main]

use user_lib::stat;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("The statistics of all syscalls:");
    stat();
    0
}
