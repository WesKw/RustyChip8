#[derive(Debug, Clone)]
pub struct Memory {
    data: Vec<u8> // memory block
}

impl Memory {
    fn new(size: u16) -> Self {
        Self {
            data: vec![0; size as usize]
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}