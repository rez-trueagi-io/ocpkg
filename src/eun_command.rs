fn run_command(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .with_context(|| format!("Failed to execute {} with args {:?}", cmd, args))?;

    if !status.success() {
        anyhow::bail!("Command {} {:?} failed with status {}", cmd, args, status);
    }

    Ok(())
}
