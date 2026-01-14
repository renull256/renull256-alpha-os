use crate::FrameBuffer;
use crate::font_data::BASIC_FONTS;

/// 文字列を描画する
/// * `fb` - フレームバッファ情報
/// * `x`, `y` - 描画開始座標
/// * `s` - 描画したい文字列
/// * `stride` - 画面の横解像度（ピクセル数）
pub fn draw_string(fb: &FrameBuffer, mut x: usize, y: usize, s: &str) {
    for c in s.chars() {
        let code = c as usize;
        if code < 128 {
            // 打ち込んだ 8x16 データを参照
            draw_char(fb, x, y, &BASIC_FONTS[code]);
            // 8x16フォントなので、次は8ピクセル横にずらす
            x += 8;
        }
    }
}

/// 1文字を描画する
fn draw_char(fb: &FrameBuffer, x: usize, y: usize, glyph: &[u8; 16]) {
    for row in 0..16 {
        let row_data = glyph[row];
        for col in 0..8 {
            // ビットが立っている（1である）箇所を塗る
            // 0x80(0b1000_0000)を右シフトしながら各ビットを確認
            if (row_data << col) & 0x80 != 0 {
                let offset = ((y + row) * fb.stride + (x + col)) * 4;
                
                // バッファオーバーフロー防止
                if offset < fb.size - 4 {
                    unsafe {
                        let p = fb.ptr.add(offset);
                        *p = 255;          // Blue
                        *(p.add(1)) = 255; // Green
                        *(p.add(2)) = 255; // Red
                    }
                }
            }
        }
    }
}