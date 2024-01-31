use std::env;
use std::process::{Command, exit, ExitStatus};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    if cfg!(target_os = "windows") {
        // 设置命令提示符执行编码为 UTF-8
        Command::new("cmd")

            .args(&["/C", "chcp", "65001"])
            .output()
            .expect("failed to execute process");
        let branch = env::args().nth(1).unwrap_or("eagle".to_string());
        let mut cnts = 100;
        loop {
            cnts -= 1;
            println!("branch = {}", branch);
            let cmd = Command::new("git")
                .args(&["push", "origin", &branch])
                .output()
                .expect("failed to execute process");
            let a = cmd.status;
            let b = cmd.stdout;
            let c = cmd.stderr;
            let output = String::from_utf8_lossy(&b).to_string();
            let err = String::from_utf8_lossy(&c).to_string();
            println!("ok {:?}", a);
            println!("output = {}", &output);
            println!("stderr = {}", &err);
            if 0 == cmd.status.code().unwrap() {
                println!("ok");
                exit(0);
            }
            let c3 = '\u{7FFF}'; // unicode字符
            if cnts == 0 {
                println!("cnts 100");
                exit(1);
            }
        }
    }
    sleep(Duration::from_secs(5));
}