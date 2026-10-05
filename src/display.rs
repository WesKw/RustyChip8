#[derive(Debug)]
pub struct Display {
    size_x: u8,
    size_y: u8,
    buffer: Vec<u8>
    // read_window: Window
}

impl Display {
    fn new(x: u8, y: u8) -> Self {
        // let win_attrs = Window::default_attributes()
        //     .with_title("Test")
        //     .with_inner_size(LogicalSize::new(x as f64, y as f64));
        Self {
            size_x: x,
            size_y: y,
            buffer: vec![0; x as usize * y as usize]
            // read_window: ActiveEventLoop::create_window(event_loop, win_attrs).unwrap()
        }
    }

    pub fn clear_buffer(&mut self) {
        self.buffer = vec![0; self.size_x as usize * self.size_y as usize];
    }

    pub fn read_buffer(&self, x: u8, y: u8) -> u8 {
        self.buffer[y as usize * self.size_x as usize + x as usize]
    }

    // write a value to the display buffer
    pub fn write_buffer(&mut self, x: u8, y: u8, value: u8) {
        self.buffer[y as usize * self.size_x as usize + x as usize] = value;
    }

    // original interpeters updated the display at 60fps, but we can get away with only redrawing the screen when the buffer is updated.
    pub fn update(&mut self) {
        // write the buffer to the display window
        // for y in 0..self.size_y {
        //     for x in 0..self.size_x {
        //         // chip8 is monochromatic
        //         let pixel = if self.read_buffer(x, y) == 0 { 0x000000 } else { 0xFFFFFF };
        //         self.read_window.set_pixel(x as usize, y as usize, pixel);
        //     }
        // }
    }
}