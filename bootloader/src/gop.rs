use log::info;
use uefi::prelude::{Boot, ResultExt, SystemTable};
use uefi::proto::console::gop::GraphicsOutput;

// A Debug value which prevents qemu from setting an insane sized value cause for
// some reason even on a 1080p display, it attempts to set the max size at about
// a 4k res
const RESIZE_1280_720: bool = true;

pub struct GopData {
    framebuffer_pointer: *mut u8,
    width: usize,
    height: usize,
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

        if RESIZE_1280_720 {
            if mode_resolution.0 == 1280 && mode_resolution.1 == 720 {
                final_mode = mode;
            }
        } else if res >= largest_size {
            final_mode = mode;
        }

        info!(">> #{} // [{}] // {:?}", index, res, mode_resolution);
    }

    // found the largest mode so set it
    info!("Largest: {:?}", final_mode.info());
    gop.set_mode(&final_mode)
        .expect_success("Failed to change GOP mode");

    info!("Gop Mode: {:#?}", gop.current_mode_info());
    info!("Framebuffer: {:#p}", gop.frame_buffer().as_mut_ptr());

    return GopData {
        framebuffer_pointer: gop.frame_buffer().as_mut_ptr(),
        width: final_mode.info().resolution().0,
        height: final_mode.info().resolution().1,
    };
}
