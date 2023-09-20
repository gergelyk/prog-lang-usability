use std::process::Command;

fn main() {

    let cmd = Command::new("ls")
            .arg("/")
            .stdout(std::process::Stdio::piped())
            .output()
            .expect("Failed to invoke command");

    if cmd.status.code() == Some(0) { // cmd.status.success() also available
        let stdout = String::from_utf8(cmd.stdout).unwrap();
        let list_files: Vec<&str> = stdout.split("\n").collect();
        println!("{:?}", list_files);
    } else {
        let stderr = String::from_utf8(cmd.stderr).unwrap();
        println!("Error: {}", stderr.trim());
    }

}
