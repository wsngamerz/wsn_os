use alloc::string::String;
use alloc::vec;
use log::*;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileMode, FileType};
use uefi::proto::media::fs::SimpleFileSystem;

pub fn read_file(file_name: &str, handle: &Handle, system_table: &SystemTable<Boot>) {
    let sfs = system_table
        .boot_services()
        .locate_protocol::<SimpleFileSystem>()
        .expect("Unable to locate Simple File System")
        .unwrap();
    let sfs = unsafe { &mut *sfs.get() };

    let mut directory = sfs.open_volume().expect("Error Opening volume").unwrap();
    let mut buffer = vec![0; 128];
    loop {
        let file_info = match directory.read_entry(&mut buffer) {
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
        info!("Root directory entry: {:?}", file_info);

        if String::from_utf16(file_info.file_name().to_u16_slice()).unwrap() == "EFI" {
            info!("Found EFI Dir");
            match File::open(
                directory.handle(),
                "EFI",
                FileMode::Read,
                FileAttribute::READ_ONLY,
            )
            .unwrap()
            .unwrap()
            .into_type()
            .unwrap()
            .unwrap()
            {
                FileType::Regular(_file) => info!("Error, our directory turned into a file"),
                FileType::Dir(directory) => {
                    let mut directory = directory;
                    let mut buffer = vec![0; 128];
                    loop {
                        let file_info = match directory.read_entry(&mut buffer) {
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
                        info!("EFI directory entry: {:?}", file_info);
                    }
                }
            }
            break;
        }
    }
}
