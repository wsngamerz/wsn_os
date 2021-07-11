use log::info;
use uefi::prelude::{Boot, ResultExt, SystemTable};
use uefi::proto::console::gop::GraphicsOutput;

pub struct GopData {
    framebuffer_pointer: *mut u8,
}

pub fn setup_gop(st: &SystemTable<Boot>) -> GopData {
    let gop = st
        .boot_services()
        .locate_protocol::<GraphicsOutput>()
        .expect_success("Failed to locate GOP");
    let gop = unsafe { &mut *gop.get() };

    info!("Available Modes: {}", gop.modes().len());

    let largest_size = 0;
    let mut final_mode = gop.modes().next().expect("GOP Mode Err").unwrap();

    // try to calculate the largest resolution we can support and get a copy of that mode
    for (index, gop_mode) in gop.modes().enumerate() {
        let mode = gop_mode.expect("GOP mode Panic");
        let mode_info = mode.info();
        let mode_resolution = mode_info.resolution();
        let res = mode_resolution.0 * mode_resolution.1;
        if res >= largest_size {
            final_mode = mode;
        }

        info!(">> #{} // [{}] // {:?}", index, res, mode_resolution);
    }

    // found the largest mode
    info!("Largest: {:?}", final_mode.info());
    gop.set_mode(&final_mode)
        .expect_success("Failed to change GOP mode");

    info!("Gop Mode: {:#?}", gop.current_mode_info());
    info!("Framebuffer: {:#p}", gop.frame_buffer().as_mut_ptr());

    return GopData {
        framebuffer_pointer: gop.frame_buffer().as_mut_ptr(),
    };
}
