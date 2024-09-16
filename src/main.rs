use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

fn main() -> io::Result<()> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: rustbin <binary_name>");
        std::process::exit(1);
    }

    let binary_name = &args[1];
    let release_binary_path = format!("target/release/{}", binary_name);
    let destination_path = format!("/usr/local/bin/{}", binary_name);

    // Check if the binary exists in the release directory
    if !Path::new(&release_binary_path).exists() {
        eprintln!("Error: {} does not exist in the target/release directory.", binary_name);
        std::process::exit(1);
    }

    // Remove any existing binary in /usr/local/bin
    if Path::new(&destination_path).exists() {
        println!("Removing existing binary from /usr/local/bin/{}", binary_name);
        fs::remove_file(&destination_path)?;
    }

    // Move the binary to /usr/local/bin
    println!("Deploying {} to /usr/local/bin", binary_name);
    fs::copy(&release_binary_path, &destination_path)?;

    // Make sure the binary is executable
    Command::new("chmod")
        .arg("+x")
        .arg(&destination_path)
        .status()
        .expect("Failed to make the binary executable");

    println!("{} has been successfully deployed to /usr/local/bin", binary_name);

    Ok(())
}

