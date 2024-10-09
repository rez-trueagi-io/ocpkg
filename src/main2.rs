use clap::{Arg, Command};
use anyhow::{Result, Context};

fn main() -> Result<()> {
    let matches = Command::new("octool")
        .version("1.0")
        .about("Installer for OpenCog Hyperon")
        .arg(Arg::new("add_repositories")
            .short('r')
            .long("add-repositories")
            .help("Add software repositories"))
        .arg(Arg::new("install_dependencies")
            .short('d')
            .long("install-dependencies")
            .help("Install build dependencies"))
        // Add other arguments as needed
        .get_matches();

    if matches.is_present("add_repositories") {
        add_repositories()?;
    }

    if matches.is_present("install_dependencies") {
        install_dependencies()?;
    }

    // Handle other arguments

    Ok(())
}
