// src/main.rs
use std::process::Command;
use clap::{App, Arg};

#[tokio::main]
async fn main() {
    let matches = App::new("OpenCog Installer")
        .version("1.0")
        .about("Installs OpenCog Hyperon")
        .arg(Arg::with_name("script_url")
            .short("u")
            .long("url")
            .takes_value(true)
            .help("URL of the ocpkg script"))
        .get_matches();

    let script_url = matches.value_of("script_url").unwrap_or("http://raw.github.com/opencog/ocpkg/master/ocpkg");

    // Download the ocpkg script
    let output = Command::new("curl")
        .arg("-L")
        .arg(script_url)
        .arg("-o")
        .arg("/usr/local/bin/octool")
        .output()
        .expect("Failed to download ocpkg script");

    if !output.status.success() {
        eprintln!("Error downloading ocpkg: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    // Make the script executable
    let output = Command::new("chmod")
        .arg("+x")
        .arg("/usr/local/bin/octool")
        .output()
        .expect("Failed to chmod ocpkg script");

    if !output.status.success() {
        eprintln!("Error setting executable permissions: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    // Execute the ocpkg script
    let output = Command::new("/usr/local/bin/octool")
        .output()
        .expect("Failed to execute ocpkg script");

    if !output.status.success() {
        eprintln!("Error executing ocpkg: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    println!("OpenCog Hyperon installation completed successfully.");
}
