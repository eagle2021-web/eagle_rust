use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    if cfg!(target_os = "windows") {
        // 设置命令提示符执行编码为 UTF-8
        Command::new("cmd")
            .args(&["/C", "chcp", "65001"])
            .output()
            .expect("failed to execute process");
        let cmd = Command::new("runas")
            .args(&["/noprofile", "/user:Administrator", "cmd", "/C", "chcp 65001"])
            .output()
            .expect("failed to execute process");

        let cmd = Command::new("setx")
            .args(&["MY_VARIABLE", "my_value2222", "/M"])
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
    } else {
        println!("This example is for Windows. For Unix-based OS, use `export`.");
    }
    sleep(Duration::from_secs(5));
}