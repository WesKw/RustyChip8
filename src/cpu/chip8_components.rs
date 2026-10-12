#[derive(Debug, Clone)]
pub struct Chip8Components {
    // timers and frequencies
    pub delay_timer: u8, // delay timer, 
    pub sound_timer: u8, // sound timer, beeps if not zero

    // registers and stack
    pub program_counter: u16, // pointer to the current instruction
    pub index_register: u16, // pointer for pointing at locations in memory
    pub stack: Vec<u16>, //stack for calling and returning from subroutines
    pub registers: [u8; 16], // registers, VF commonly used as flag register
}

impl Default for Chip8Components {
    fn default() -> Self {
        Self {
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0,
            index_register: 0,
            stack: vec![0; 16],
            registers: [0; 16],
        }
    }
}