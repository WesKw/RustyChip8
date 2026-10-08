#[derive(Debug, Clone)]
pub struct Chip8Components {
    // timers and frequencies
    pub delay_timer: u8, // delay timer, 
    pub delay_timer_freq: u8, // delay timer frequency
    pub sound_timer: u8, // sound timer, beeps if not zero
    pub sound_timer_freq: u8, // sound timer frequency

    // registers and stack
    pub program_counter: u16, // pointer to the current instruction
    pub index_register: u16, // pointer for pointing at locations in memory
    pub stack: [u8; 16], //stack for calling and returning from subroutines
    pub registers: [u8; 16], // registers, VF commonly used as flag register
}

impl Default for Chip8Components {
    fn default() -> Self {
        Self {
            delay_timer: 0,
            delay_timer_freq: 60,
            sound_timer: 0,
            sound_timer_freq: 60,
            program_counter: 0,
            index_register: 0,
            stack: [0; 16],
            registers: [0; 16],
        }
    }
}