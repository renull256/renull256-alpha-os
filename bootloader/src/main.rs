#![no_main]
#![no_std]

use core::fmt::Write;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use kernel::{kernel_main, FrameBuffer};

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // UEFIのログサービスを初期化
    uefi_services::init(&mut system_table).unwrap();

    // 画面をクリア
    system_table.stdout().clear().unwrap();

    // 文字を表示
    writeln!(system_table.stdout(), "Booting now...\r\n");

    let (fb_ptr, fb_size) = {
        // GOP (Graphics Output Protocol) を取得
        let gop_handle = system_table.boot_services().get_handle_for_protocol::<GraphicsOutput>().unwrap();
        let mut gop = system_table.boot_services().open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();
        
        // フレームバッファの情報を取得
        let mut fb = gop.frame_buffer();
        let fb_ptr = fb.as_mut_ptr();
        let fb_size = fb.size();

        (fb_ptr, fb_size)
    };

    let fb = FrameBuffer {
        ptr: fb_ptr,
        size: fb_size,
    };

    // 5秒待機
    system_table.boot_services().stall(3_000_000);
    
    // run kernel
    kernel_main(fb);

    Status::SUCCESS
}
