use alloc::string::String;
use alloc::vec;
use log::*;
use uefi::prelude::*;
use uefi::proto::media::file::{Directory, File, FileHandle, FileProtocolInfo, RegularFile};
use uefi::proto::media::fs::SimpleFileSystem;

pub fn read_kernel(_handle: &Handle, system_table: &SystemTable<Boot>) {
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
    let efi_dir = open_dir(directory, "EFI");
    let wsn_dir = open_dir(efi_dir, "WSNOS");
    let wsn_kernel = open_file(wsn_dir, "kernel.bin");
}

fn open_dir(root_dir: Directory, dir_name: &str) -> Directory {
    match open_file_directory(root_dir, dir_name)
        .unwrap()
        .into_type()
        .unwrap()
        .unwrap()
    {
        uefi::proto::media::file::FileType::Dir(dir) => Some(dir),
        uefi::proto::media::file::FileType::Regular(_) => None,
    }
    .unwrap()
}

fn open_file(root_dir: Directory, file_name: &str) -> RegularFile {
    match open_file_directory(root_dir, file_name)
        .unwrap()
        .into_type()
        .unwrap()
        .unwrap()
    {
        uefi::proto::media::file::FileType::Dir(_) => None,
        uefi::proto::media::file::FileType::Regular(file) => Some(file),
    }
    .unwrap()
}

fn open_file_directory(mut root_dir: Directory, entry_name: &str) -> Option<FileHandle> {
    let mut buffer = vec![0; 128];

    loop {
        let file_info = match root_dir.read_entry(&mut buffer) {
            Ok(completion) => {
                if let Some(info) = completion.unwrap() {
                    info
                } else {
                    // We've reached the end of the directory
                    break;
                }
            }
            Err(error) => {
                // Buffer is not big enough, allocate a bigger one and try again.
                let min_size = error.data().unwrap();
                buffer.resize(min_size, 0);
                continue;
            }
        };

        // if the entry is the one we are looking for:
        if String::from_utf16(file_info.file_name().to_u16_slice()).unwrap() == entry_name {
            return Some(
                File::open(
                    root_dir.handle(),
                    entry_name,
                    uefi::proto::media::file::FileMode::Read,
                    uefi::proto::media::file::FileAttribute::READ_ONLY,
                )
                .unwrap()
                .unwrap(),
            );
        }
    }

    return None;
}
