#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

extern crate alloc;
extern crate log;
extern crate rlibc;

use log::info;
use uefi::prelude::*;

mod gop;
mod load;

#[entry]
fn efi_main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Initialize utilities (logging, memory allocation...)
    uefi_services::init(&mut system_table).expect_success("Failed to initialize utilities");

    system_table
        .stdout()
        .reset(false)
        .expect_success("Failed to reset stdout");
    info!("Hello World");

    display_uefi_info(&system_table);
    let gop_data = gop::setup_gop(&system_table);
    load::read_file("EFI/WSNOS/Kernel.bin", &handle, &system_table);

    loop {
        // clear interrupts and halt to stop 100% cpu usage
        unsafe { asm!("cli; hlt") }
    }
    return Status::SUCCESS;
}

fn display_uefi_info(st: &SystemTable<Boot>) {
    let revision = st.uefi_revision();
    let major = revision.major();
    let minor = revision.minor();

    info!("UEFI {}.{}", major, minor);
}
