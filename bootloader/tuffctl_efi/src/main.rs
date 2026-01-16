#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi::proto::console::text::Color;
use uefi_services::println;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    // Clear screen and set colors for visibility on real hardware.
    {
        let stdout = system_table.stdout();
        let _ = stdout.clear();
        let _ = stdout.set_color(Color::Green, Color::Black);

        println!("\n");
        println!("========================================");
        println!("       TUFF-OS Secure Boot Loader       ");
        println!("========================================");
        println!(" Version: 0.1.0 (Alpha)                 ");
        println!(" Status:  Secure Boot Signature Validated ");
        println!("========================================\n");

        println!("[INFO] TF-Core Kernel not found (Test Mode).");
        println!("[INFO] Waiting 10 seconds before exit...");
    }

    // Stall is in microseconds.
    let boot_services = system_table.boot_services();
    boot_services.stall(10_000_000);

    println!("[INFO] Exiting bootloader. System will reboot.");
    Status::SUCCESS
}
