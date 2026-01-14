#![no_main]
#![no_std]

//////////////////
//  Bootloader  //
//////////////////

use core::fmt::Write;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use kernel::{kernel_main, FrameBuffer};

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // init UEFI log service
    uefi_services::init(&mut system_table).unwrap();

    // clear display
    system_table.stdout().clear().unwrap();

    // print sentences
    writeln!(system_table.stdout(), "Booting now...\r\n");

    let (fb_ptr, fb_size, stride) = {
        // get GOP (Graphics Output Protocol)
        let gop_handle = system_table.boot_services().get_handle_for_protocol::<GraphicsOutput>().unwrap();
        let mut gop = system_table.boot_services().open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();
        let mode_info = gop.current_mode_info();
        
        // get frame buffer infomation
        let mut fb = gop.frame_buffer();
        let fb_ptr = fb.as_mut_ptr();
        let fb_size = fb.size();
        let stride = mode_info.stride();

        (fb_ptr, fb_size, stride)
    };

    let fb = FrameBuffer {
        ptr: fb_ptr,
        size: fb_size,
        stride: stride
    };

    // wait for 3 seconds
    system_table.boot_services().stall(3_000_000);
    
    // run kernel
    kernel_main(fb);

    Status::SUCCESS
}
