use clap::{Parser, Subcommand};
use anyhow::{Result, bail};
use rand::RngCore;
use std::io::{self, Write};

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
    println!("*** TUFF-FS INITIALIZATION ***");

    // 0. Pre-check System UUID
    let sys_uuid = match usb_storage::UsbKeyStore::get_system_uuid() {
        Ok(u) => u,
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            bail!("System UUID check failed. Cannot bind key to this hardware.");
        }
    };
    println!("Target System UUID: {}", sys_uuid);

    // 1. Generate Key
    println!("Generating 256-bit Master Key...");
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    let key_hex = hex::encode(key).to_uppercase();

    // 2. Display Warning & Hex Grid
    print!("\x1B[2J\x1B[1;1H"); // Clear Screen
    println!("================================================================");
    println!("                    [ TUFF-FS MASTER KEY ]");
    println!("================================================================");
    println!(" WARNING: This key is the ONLY way to recover your data.");
    println!("          If you lose this, your data is PERMANENTLY LOST.");
    println!("          TAKE A PHOTO OF THIS SCREEN NOW.");
    println!("================================================================\n");

    let chunks = key_hex.as_bytes().chunks(4);
    for (i, chunk) in chunks.enumerate() {
        if i % 4 == 0 { print!("Line {}:  ", (i/4)+1); }
        print!("{}     ", std::str::from_utf8(chunk).unwrap());
        if (i+1) % 4 == 0 { println!("\n"); }
    }
    println!("================================================================\n");

    // 3. Verification Logic (Corner Check)
    println!("[VERIFICATION REQUIRED]");
    println!("Check your photo/memo and enter the requested key parts.\n");

    let first_4 = &key_hex[0..4];
    let last_4 = &key_hex[60..64];

    let input_start = prompt("1. Enter Line 1, Group 1 (Top-Left)     : ")?;
    let input_end   = prompt("2. Enter Line 4, Group 4 (Bottom-Right) : ")?;

    if input_start.trim().to_uppercase() != first_4 ||
       input_end.trim().to_uppercase() != last_4 {
        println!("\n[FAILURE] Key mismatch. Initialization ABORTED. Key discarded.");
        bail!("Key verification failed");
    }

    println!("\n[SUCCESS] Key verified.");

    // 4. USB Selection & Write
    println!("Scanning for USB devices...");
    let target_usb = usb_storage::UsbKeyStore::select_usb_device()?;

    println!("Writing key to {:?} for UUID {}...", target_usb, sys_uuid);
    usb_storage::UsbKeyStore::write_key_to_usb(&target_usb, &key, &sys_uuid)?;

    println!("[SUCCESS] Key saved to USB. Ready to format TUFF-FS volumes (TODO).");

    Ok(())
}

fn prompt(msg: &str) -> Result<String> {
    print!("{}", msg);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
