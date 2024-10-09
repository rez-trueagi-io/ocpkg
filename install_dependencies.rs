use std::process::Command;
use anyhow::{Result, Context};

fn install_dependencies() -> Result<()> {
    install_python()?;

    let message = "Installing OpenCog build dependencies....";
    println!("[INFO] {}", message);

    let status = Command::new("sudo")
        .args(&["apt-get", "install", "-y", "--no-install-recommends", "build-essential", "cmake", /* other packages */])
        .status()
        .context("Failed to execute apt-get install")?;

    if !status.success() {
        anyhow::bail!("apt-get install failed");
    }

    ensure_utf_installed()?;
    update_ccache_links()?;
    install_guile()?;

    Ok(())
}
