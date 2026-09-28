use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    program: String, // program file path

    #[arg(long, default_value_t=64)]
    screen_size_x: u8, // horizontal screen size
    #[arg(long, default_value_t=32)]
    screen_size_y: u8, // vertical screen size

    // #[arg(short, long, default_value_t=1)]
    // clock_speed_mhz: u8
    #[arg(long, default_value_t=700)]
    instructions_per_second: u32 // instructions per second
}

struct Chip8Components {
    // general components
    display_x: u8, // default is 64 pixels
    display_y: u8, // default is 32 pixels
    memory_size: u16, // default memory size is 4KB -> 4096B
    memory: Box<[u8; 4096]>, // array of memory? 

    // timers and frequencies
    delay_timer: u8, // delay timer, 
    delay_timer_freq: u8, // delay timer frequency
    sound_timer: u8, // sound timer, beeps if not zero
    sound_timer_freq: u8, // sound timer frequency

    // registers and stack
    program_counter: u16, // pointer to the current instruction
    index_register: u16, // pointer for pointing at locations in memory
    stack: [u16; 16], //stack for calling and returning from subroutines
    registers: [u16; 16], // registers, VF commonly used as flag register
}

impl Default for Chip8Components {
    fn default() -> Self {
        Self {
            display_x: 64,
            display_y: 32,
            memory_size: 4096,
            memory: Box::new([0; 4096]),
            delay_timer: 0,
            delay_timer_freq: 60,
            sound_timer: 0,
            sound_timer_freq: 60,
            program_counter: 0,
            index_register: 0,
            stack: [0; 16],
            registers: [0; 16]
        }
    }
}


fn fetch() {

}

fn decode() {

}

fn execute() {

}

// Initialize Chip8.
fn initialize() {
    // load default components of Chip8
    println!("Initializing Chip8");

    // load program into memory

    // set PC to first instruction in memory
}

// Fetch/decode/execute loop
fn main_loop() {
    loop {
        break;
    }
}

// shutdown Chip8
fn finalize() {

}

// Main entry point
fn main() {
    let args = Args::parse();

    println!("Program file: {}", args.program);
    println!("Using screen size {}x{}", args.screen_size_x, args.screen_size_y);

    let mut components = Chip8Components::default();

    initialize();
    main_loop();
    finalize();
}
