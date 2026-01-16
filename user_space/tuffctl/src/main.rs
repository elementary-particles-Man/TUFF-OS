use clap::{Parser, Subcommand};
use anyhow::{Result, bail};
use rand::RngCore;

mod usb_storage;

#[derive(Parser)]
#[command(name = "tuffctl")]
#[command(about = "TUFF-OS Management CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Commit,
    Truncate,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => run_init()?,
        _ => println!("Not implemented yet"),
    }
    Ok(())
}

fn run_init() -> Result<()> {
    // 1. Generate Key
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    let key_hex = hex::encode(key).to_uppercase();

    // 2. Display Warning & Hex Grid (UI Logic Omitted for brevity, assume previous impl)
    println!("*** TUFF-FS INITIALIZATION ***");
    println!("Key Generated: {}...", &key_hex[0..8]);

    // 3. Detect & Write to USB
    println!("Scanning for USB devices...");
    let usbs = usb_storage::UsbKeyStore::find_usb_devices()?;
    if usbs.is_empty() {
        bail!("No USB device found. Insert a USB drive to save the key.");
    }

    let target_usb = &usbs[0]; // Auto-pick first for now
    println!("Found USB: {:?}", target_usb);
    println!("Writing key to USB...");

    // Mock UUID
    let sys_uuid = "00000000-0000-0000-0000-000000000001";
    usb_storage::UsbKeyStore::write_key_to_usb(target_usb, &key, sys_uuid)?;

    println!("[SUCCESS] Key saved to USB. Formatting TUFF-FS...");

    Ok(())
}
