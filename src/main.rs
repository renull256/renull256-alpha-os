#![no_main]
#![no_std]

use core::fmt::Write;
use uefi::prelude::*;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // UEFIのログサービスを初期化
    uefi_services::init(&mut system_table).unwrap();

    // 画面をクリア
    system_table.stdout().clear().unwrap();

    // 文字を表示
    writeln!(system_table.stdout(), "Hello, Rust UEFI Bootloader!");

    // 5秒待機して終了
    system_table.boot_services().stall(5_000_000);

    Status::SUCCESS
}
