use std::process::Command;


fn main() -> std::io::Result<()> {
    let mut cmd = Command::new("sss");
    let _b = cmd.status().unwrap();
    Ok(())
}