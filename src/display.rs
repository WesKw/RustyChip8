use minifb::{Key, Window, WindowOptions};

#[derive(Debug)]
pub struct Display {
    size_x: u32,
    size_y: u32,
    buffer: Vec<u32>,
    window: Window,
}

impl Display {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            size_x: x,
            size_y: y,
            buffer: vec![0; x as usize * y as usize],
            window: Window::new(
                "WIndow Test",
                x as usize,
                y as usize,
                WindowOptions::default(),
            ).unwrap_or_else(|e| {
                panic!("{}", e)
            }),
        }
    }

    pub fn clear_buffer(&mut self) {
        self.buffer = vec![0x0; self.size_x as usize * self.size_y as usize];
    }

    pub fn read_buffer(&self, x: u32, y: u32) -> u32 {
        self.buffer[y as usize * self.size_x as usize + x as usize]
    }

    // write a value to the display buffer
    pub fn write_buffer(&mut self, x: u32, y: u32, value: u32) {
        let value = if value > 0 { 0xFFFFFFFF } else { 0x0 };
        self.buffer[y as usize * self.size_x as usize + x as usize] = value;
    }

    // original interpeters updated the display at 60fps, but we can get away with only redrawing the screen when the buffer is updated.
    pub fn update(&mut self) {
        // write the buffer to the display window
        let _ = self.window.update_with_buffer(&self.buffer, self.size_x as usize, self.size_y as usize).unwrap();
        // for y in 0..self.size_y {
        //     for x in 0..self.size_x {
        //         // chip8 is monochromatic
        //         let pixel = if self.read_buffer(x, y) == 0 { 0x000000 } else { 0xFFFFFF };
        //         self.read_window.set_pixel(x as usize, y as usize, pixel);
        //     }
        // }
    }
}