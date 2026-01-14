use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // TTFファイルを指定
    let font_bytes = include_bytes!("src/fonts/JetBrainsMono-Regular.ttf");
    let font = fontdue::Font::from_bytes(font_bytes as &[u8], fontdue::FontSettings::default()).unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("font_data.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(f, "pub const BASIC_FONTS: [[u8; 16]; 128] = [").unwrap();

    for i in 0..128 {
        let c = i as u8 as char;
        // 13.0〜14.0px が 8x16 の枠には最も綺麗に収まります
        let (metrics, bitmap) = font.rasterize(c, 14.0);

        let mut glyph = [0u8; 16];

        for y in 0..metrics.height {
            for x in 0..metrics.width {
                if bitmap[y * metrics.width + x] > 100 {
                    let baseline = 12;
                    let py = baseline - metrics.bounds.ymin as i32 - metrics.height as i32 + y as i32;
                    let px = x as i32 + (8 - metrics.width as i32) / 2; // 水平中央寄せ

                    if py >= 0 && py < 16 && px >= 0 && px < 8 {
                        glyph[py as usize] |= 0x80 >> px;
                    }
                }
            }
        }
        writeln!(f, "    {:?}, // {:?}", glyph, c).unwrap();
    }
    writeln!(f, "\n];").unwrap();
}