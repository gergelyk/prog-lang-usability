use std::arch::asm;

fn main() {
    let mut pid: u32;
    unsafe {
        asm!(
            "mov eax, $20",
            "int $0x80",
            "mov {pid:e}, eax",
             pid = out(reg) pid,
        );
    }
    println!("{}", pid);
}
