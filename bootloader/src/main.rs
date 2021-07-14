#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

extern crate alloc;
extern crate log;
extern crate rlibc;

use alloc::vec;
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
    // setup graphics
    let _gop_data = gop::setup_gop(&system_table);
    // load the kernel
    let kernel_info = load::read_kernel(&handle, &system_table);

    info!("Kernel Header >>> {:?}", kernel_info);
    info!("Entry? >> {:x}", kernel_info.elf_header.e_entry);

    // TODO: Memory allocations and passing data to the kernel

    // leave uefi boot services
    info!("Leaving Boot services");
    let memory_map_buffer = &mut vec![0; system_table.boot_services().memory_map_size() + 8];
    let _memory_map = system_table.exit_boot_services(handle, memory_map_buffer);

    // jump to the kernel
    // NOTE: This doesnt actually work, shame :(
    unsafe {
        asm!("jmp [{0:r}];", in(reg) kernel_info.elf_header.e_entry);
    }

    info!("After Kernel Jump, how are you here");

    loop {
        // clear interrupts and halt to stop 100% cpu usage
        unsafe { asm!("cli; hlt") }
    }
}

fn display_uefi_info(st: &SystemTable<Boot>) {
    let revision = st.uefi_revision();
    let major = revision.major();
    let minor = revision.minor();

    info!("UEFI {}.{}", major, minor);
}
