use std::process::Command;
use anyhow::Result;

fn build_cli() -> Result<()> {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .spawn()?
        .wait()?;
    Ok(())
}

fn test_cli() -> Result<()> {
    Command::new("cargo")
        .arg("test")
        .spawn()?
        .wait()?;
    Ok(())
}

fn main() -> Result<()> {
    // You can call the tasks like `build_cli()` or `test_cli()` based on input
    build_cli()?;
    Ok(())
}
