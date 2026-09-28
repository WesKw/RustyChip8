struct Chip8Components {
    // general components
    display_x: u8, // default is 64 pixels
    display_y: u8, // default is 32 pixels
    memory_size: u16, // default memory size is 4KB -> 4096B
    memory: Box<u8>, // memory allocation?

    // timers and frequencies
    delay_timer: u8, // delay timer, 
    delay_timer_freq: u8, // delay timer frequency
    sound_timer: u8, // sound timer, beeps if not zero
    sound_timer_freq: u8, // sound timer frequency

    // registers and stack
    program_counter: *u16, // pointer to the current instruction
    index_register: *u16, // pointer for pointing at locations in memory
    stack: Vec<*u16>, //stack for calling and returning from subroutines
    registers: Vec<*u16>, // registers, VF commonly used as flag register
    // rV0: *u16,
    // rV1: *u16,
    // rV2: *u16,
    // rV3: *u16,
    // rV4: *u16,
    // rV5: *u16,
    // rV6: *u16,
    // rV7: *u16,
    // rV8: *u16,
    // rV9: *u16,
    // rVA: *u16,
    // rVB: *u16,
    // rVC: *u16,
    // rVD: *u16,
    // rVE: *u16,
    // rVF: *u16 // flag register
}

impl Default for Chip8Components {
    fn default() -> Self {
        Self {
            display_x: 64,
            display_y: 32,
            memory_size: 4096,

        }
    }
}