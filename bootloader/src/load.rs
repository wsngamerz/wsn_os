use alloc::string::String;
use alloc::vec;
use log::*;
use uefi::prelude::*;
use uefi::proto::media::file::{Directory, File, FileHandle, RegularFile};
use uefi::proto::media::fs::SimpleFileSystem;

#[derive(Debug)]
struct MyFileInfo {
    file_size: u64,
    physical_size: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ELFIdent {
    ei_mag: [u8; 4],
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    ei_pad: [u8; 7],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ELFHeader {
    pub e_ident: ELFIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    pub e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[derive(Debug)]
pub struct KernelInfo {
    pub elf_header: ELFHeader,
}

pub fn read_kernel(_handle: &Handle, system_table: &SystemTable<Boot>) -> KernelInfo {
    // load the simple file system protocol
    let sfs = system_table
        .boot_services()
        .locate_protocol::<SimpleFileSystem>()
        .expect("Unable to locate Simple File System")
        .unwrap();
    let sfs = unsafe { &mut *sfs.get() };

    // TODO: Turn into a function where open(volume, "EFI/WSNOS/kernel.bin") is possible
    // open FS0://EFI/WSNOS/kernel.bin
    let directory = sfs.open_volume().expect("Error Opening volume").unwrap();

    let (efi_dir, _efi_dir_info) = open_dir(directory, "EFI");
    let (wsn_dir, _wsn_dir_info) = open_dir(efi_dir, "WSNOS");
    let (mut wsn_kernel, wsn_kernel_info) = open_file(wsn_dir, "kernel.bin");

    // info!("WSN Kernel: {:?}", wsn_kernel_info);

    // load into memory
    let buffer = &mut vec![0; wsn_kernel_info.file_size as usize];
    let read_bytes = match wsn_kernel.read(buffer) {
        Ok(completion) => Some(completion.unwrap()),
        Err(error) => {
            error!("Error: {:?}", error);
            None
        }
    }
    .unwrap();
    trace!("Read {} Bytes", read_bytes);

    // read elf header
    let kernel_elf_header_buffer = &buffer[..64];
    trace!("Kernel Header Bytes>> {:02x?}", kernel_elf_header_buffer);
    let (head, body, _tail) = unsafe { kernel_elf_header_buffer.align_to::<ELFHeader>() };
    assert!(head.is_empty(), "Data was not aligned");
    let kernel_elf_header = &body[0];

    // info!("{:#?}", kernel_elf_header);

    // return info so we can jump into it
    return KernelInfo {
        elf_header: *kernel_elf_header,
    };
}

fn open_dir(root_dir: Directory, dir_name: &str) -> (Directory, MyFileInfo) {
    let (dir, info) = open_file_directory(root_dir, dir_name).unwrap();
    (
        match dir.into_type().unwrap().unwrap() {
            uefi::proto::media::file::FileType::Dir(dir) => Some(dir),
            uefi::proto::media::file::FileType::Regular(_) => None,
        }
        .unwrap(),
        info,
    )
}

fn open_file(root_dir: Directory, file_name: &str) -> (RegularFile, MyFileInfo) {
    let (file, info) = open_file_directory(root_dir, file_name).unwrap();
    (
        match file.into_type().unwrap().unwrap() {
            uefi::proto::media::file::FileType::Dir(_) => None,
            uefi::proto::media::file::FileType::Regular(file) => Some(file),
        }
        .unwrap(),
        info,
    )
}

fn open_file_directory(
    mut root_dir: Directory,
    entry_name: &str,
) -> Option<(FileHandle, MyFileInfo)> {
    // loop until found
    loop {
        // let mut buffer: [u8; 128] = [0; 128];

        let mut buffer = vec![0; 128];

        let file_info = match root_dir.read_entry(&mut buffer) {
            Ok(completion) => {
                if let Some(info) = completion.unwrap() {
                    info
                } else {
                    // We've reached the end of the directory
                    break;
                }
            }
            Err(_error) => {
                // Buffer is not big enough
                error!("Buffer too small");
                continue;
            }
        };

        // if the entry is the one we are looking for:
        if String::from_utf16(file_info.file_name().to_u16_slice()).unwrap() == entry_name {
            return Some((
                File::open(
                    root_dir.handle(),
                    entry_name,
                    uefi::proto::media::file::FileMode::Read,
                    uefi::proto::media::file::FileAttribute::READ_ONLY,
                )
                .unwrap()
                .unwrap(),
                MyFileInfo {
                    file_size: file_info.file_size(),
                    physical_size: file_info.physical_size(),
                },
            ));
        }
    }

    return None;
}
