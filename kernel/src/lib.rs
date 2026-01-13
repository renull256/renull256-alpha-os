#![no_std]

pub struct FrameBuffer {
    pub ptr: *mut u8,
    pub size: usize,
}

pub fn kernel_main(fb: FrameBuffer) -> ! {
    unsafe {
        for i in 0..(fb.size / 4) {
            let pixel = fb.ptr.add(i * 4);
            *pixel = 255;           // B
            *pixel.add(1) = 255;    // G
            *pixel.add(2) = 0;      // R
        }
    }

    // NOP
    loop {
        core::hint::spin_loop();
    }
    
}