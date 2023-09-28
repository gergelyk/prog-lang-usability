use syscalls::{Sysno, syscall};

fn main() {
    match unsafe { syscall!(Sysno::getpid) } {
        Ok(pid) => {
            println!("{}", pid);
        }
        Err(err) => {
            eprintln!("getpid() failed: {}", err);
        }
    }
}
