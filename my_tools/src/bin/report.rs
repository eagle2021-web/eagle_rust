use std::process::Command;
use color_eyre::Report; // Make sure color-eyre is compatible with anyhow.


fn main() -> Result<(), Report> {
    color_eyre::install()?; // Initialize color-eyre.
    let mut cmd = Command::new("sss");
    let _b = cmd.status().unwrap();
    Ok(())
}