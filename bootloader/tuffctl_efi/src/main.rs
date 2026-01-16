#![no_std]
#![no_main]

use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    log::info!("TUFF-OS Bootloader (tuffctl.efi) loaded.");

    // TODO: Verify TF-Core image signature

    Status::SUCCESS
}
