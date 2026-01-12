#![no_main]
#![no_std]

use core::fmt::Write;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // UEFIのログサービスを初期化
    uefi_services::init(&mut system_table).unwrap();

    // 画面をクリア
    system_table.stdout().clear().unwrap();

    // 文字を表示
    writeln!(system_table.stdout(), "Hello, Rust UEFI Bootloader!\r\n");

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

    // 直接メモリを叩いて四角形を描画する
    unsafe {
        // 画面の左上に 200x200 の白い正方形を描く
        for y in 0..200 {
            for x in 0..200 {
                // ピクセルは4バイト
                // 簡易的に 800 を掛けて位置を計算
                let offset = (y * 1024 + x) * 4;
                if offset < fb_size - 4 {
                    let pixel = fb_ptr.add(offset);
                    *pixel = 255;       // B
                    *pixel.add(1) = 255; // G
                    *pixel.add(2) = 255; // R
                }
            }
        }
    }

    // 5秒待機
    system_table.boot_services().stall(5_000_000);

    // Exit Boot Services
    let (_runtime_table, _mmap_iter) = system_table.exit_boot_services();

    // OSに権限移行済み
    unsafe {
        for i in 0..(fb_size / 4) {
            let pixel = fb_ptr.add(i as usize * 4);
            *pixel = 150;       // B
            *pixel.add(1) = 50;  // G
            *pixel.add(2) = 50;  // R
        }
    }

    // NOP
    loop {
        core::hint::spin_loop();
    }

    Status::SUCCESS
}
