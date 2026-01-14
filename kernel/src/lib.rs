#![no_std]

mod font;
pub mod font_data {
    include!(concat!(env!("OUT_DIR"), "/font_data.rs"));
}

pub struct FrameBuffer {
    pub ptr: *mut u8,
    pub size: usize,
    pub stride: usize,  // the number of horizontal length in the display
}

// Entry point
pub fn kernel_main(fb: FrameBuffer) -> ! {
    unsafe {
        for i in 0..(fb.size / 4) {
            let pixel = fb.ptr.add(i * 4);
            *pixel = 0;           // B
            *pixel.add(1) = 0;    // G
            *pixel.add(2) = 0;      // R
        }
    }

    font::draw_string(&fb, 20, 30, "Hello renull-alpha-os!");

    // NOP
    loop {
        core::hint::spin_loop();
    }
    
}

