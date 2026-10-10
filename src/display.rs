use minifb::{Window, WindowOptions, Scale, ScaleMode};

#[derive(Debug)]
pub struct Display {
    size_x: u32,
    size_y: u32,
    internal_buffer: Vec<bool>, // use booleans for a monochromatic screen instead of bytes
    display_buffer: Vec<u32>,
    window: Window,
}

impl Display {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            size_x: x,
            size_y: y,
            internal_buffer: vec![false; x as usize * y as usize],
            display_buffer: vec![0; x as usize * y as usize],
            window: Window::new(
                "Window Test",
                x as usize,
                y as usize,
                WindowOptions {
                    borderless: false,
                    title: true,
                    resize: true,
                    scale: Scale::X16,
                    scale_mode: ScaleMode::AspectRatioStretch,
                    topmost: false,
                    transparency: false,
                    none: false,
                },
            ).unwrap_or_else(|e| {
                panic!("{}", e)
            }),
        }
    }

    pub fn clear_buffer(&mut self) {
        self.internal_buffer = vec![false; self.size_x as usize * self.size_y as usize];
        self.display_buffer = vec![0x0; self.size_x as usize * self.size_y as usize];
    }

    pub fn read_buffer(&self, x: u32, y: u32) -> bool {
        self.internal_buffer[y as usize * self.size_x as usize + x as usize]
    }

    // write a value to the display buffer
    pub fn write_buffer(&mut self, x: u32, y: u32, value: bool) {
        // let value = if value > 0 { true } else { false };
        self.internal_buffer[y as usize * self.size_x as usize + x as usize] = value;
    }

    // update the display buffer based on the monochromatic internal buffer
    pub fn update_display_buffer(&mut self) {
        for (index, value) in self.internal_buffer.iter().enumerate() {
            let display_value = if *value == true { 0x00FFFFFF } else { 0x0 };
            self.display_buffer[index] = display_value;
        }
    }

    // original interpeters updated the display at 60fps, but we can get away with only redrawing the screen when the buffer is updated.
    pub fn update(&mut self) {
        // write the buffer to the display window
        self.update_display_buffer();
        let _ = self.window.update_with_buffer(&self.display_buffer, self.size_x as usize, self.size_y as usize).unwrap();
        // for y in 0..self.size_y {
        //     for x in 0..self.size_x {
        //         // chip8 is monochromatic
        //         let pixel = if self.read_buffer(x, y) == 0 { 0x000000 } else { 0xFFFFFF };
        //         self.read_window.set_pixel(x as usize, y as usize, pixel);
        //     }
        // }
    }
}