#![no_std]
#![no_main]
#![feature(abi_efiapi)]

extern crate log;
extern crate rlibc;

use log::info;
use uefi::prelude::*;

#[entry]
fn efi_main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Initialize utilities (logging, memory allocation...)
    uefi_services::init(&mut system_table).expect_success("Failed to initialize utilities");

    system_table
        .stdout()
        .reset(false)
        .expect_success("Failed to reset stdout");
        
    info!("Hello World");

    display_uefi_info(&system_table);
    setup_gop(&system_table);

    loop {}
    return Status::SUCCESS;
}

fn display_uefi_info(st: &SystemTable<Boot>) {
    let revision = st.uefi_revision();
    let major = revision.major();
    let minor = revision.minor();

    info!("UEFI {}.{}", major, minor);
}

fn setup_gop(st: &SystemTable<Boot>) {
    use uefi::proto::console::gop::GraphicsOutput;

    let protocol = st
        .boot_services()
        .locate_protocol::<GraphicsOutput>()
        .unwrap().unwrap();
    let gop = unsafe { &mut *protocol.get() };

    info!("       Gop Mode: {:?}", gop.current_mode_info());
    info!("    Framebuffer: {:#p}", gop.frame_buffer().as_mut_ptr());
    info!("Available Modes: {}", gop.modes().len());

    // gop.modes().for_each({

    // })
}
