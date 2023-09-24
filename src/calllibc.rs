#![feature(rustc_private)]
extern crate libc;
use libc::pid_t;

#[link(name = "c")]
extern {
    fn getpid() -> pid_t;
}

fn main() {
    let pid = unsafe { getpid() };
    println!("{}", pid);
}
