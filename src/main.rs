use clap::Parser;
use std::fs;
use std::io;
use winit::dpi::LogicalSize;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::Window;

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    program: String, // program file path

    #[arg(long, default_value_t=4096)]
    memory_size: u16, // total memory capacity

    #[arg(long, default_value_t=64)]
    screen_size_x: u8, // horizontal screen size
    #[arg(long, default_value_t=32)]
    screen_size_y: u8, // vertical screen size

    // #[arg(short, long, default_value_t=1)]
    // clock_speed_mhz: u8
    #[arg(long, default_value_t=700)]
    instructions_per_second: u32 // instructions per second
}

#[derive(Debug)]
struct Display {
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

    fn read_buffer(&self, x: u8, y: u8) -> u8 {
        self.buffer[y as usize * self.size_x as usize + x as usize]
    }

    // write a value to the display buffer
    fn write_buffer(&mut self, x: u8, y: u8, value: u8) {
        self.buffer[y as usize * self.size_x as usize + x as usize] = value;
    }

    // original interpeters updated the display at 60fps, but we can get away with only redrawing the screen when the buffer is updated.
    fn update_display(&mut self) {
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

#[derive(Debug, Clone)]
struct Memory {
    data: Vec<u8> // memory block
}

impl Memory {
    fn new(size: u16) -> Self {
        Self {
            data: vec![0; size as usize]
        }
    }

    fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}

#[derive(Debug, Clone)]
struct Chip8Components {
    // timers and frequencies
    delay_timer: u8, // delay timer, 
    delay_timer_freq: u8, // delay timer frequency
    sound_timer: u8, // sound timer, beeps if not zero
    sound_timer_freq: u8, // sound timer frequency

    // registers and stack
    program_counter: u16, // pointer to the current instruction
    index_register: u16, // pointer for pointing at locations in memory
    stack: [u16; 16], //stack for calling and returning from subroutines
    registers: [u8; 16], // registers, VF commonly used as flag register
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
            registers: [0; 16]
        }
    }
}


fn fetch(components: &mut Chip8Components, memory: &Memory) -> u16 {
    // get the next instruction from memory
    // instruction is 2 bytes
    println!("Reading instruction at PC: {:x}", components.program_counter);
    let byte1: u16 = (memory.read(components.program_counter) as u16) << 8; // shift over one byte
    let byte2: u16 = memory.read(components.program_counter + 1) as u16;
    let insn: u16 = byte1 | byte2;

    // update program counter
    components.program_counter += 2;

    // return the insn
    insn
}   

fn decode(bytes: u16) {
    println!("Got instruction {:x}", bytes);
    match bytes {
        _ => println!("Unknown instruction {:x}", bytes)
    }
}

fn execute() {

}

// Initialize Chip8.
fn initialize(components: &mut Chip8Components, memory: &mut Memory, args: &Args) {
    // load default components of Chip8
    println!("Initializing Chip8");

    // load program into memory
    let prog: Vec<u8> = fs::read(&args.program).expect("Failed to read program file");
    let start: u16 = 0x200; // typical start address for Chip8 programs
    for (i, &byte) in prog.iter().enumerate() {
        memory.write(start + i as u16, byte);
    }

    // set PC to first instruction in memory
    components.program_counter = start;

    // generate window
}

// Fetch/decode/execute loop
fn main_loop(components: &mut Chip8Components, memory: &mut Memory, display: &mut Display) {
    loop {
        let instruction_bytes: u16 = fetch(components, memory);

        // temporary break if we hit an empty instruction
        if instruction_bytes == 0 {
            break;
        }

        let instruction = decode(instruction_bytes);
        // execute(instruction);

        // update program counter
        // components.program_counter += 2;
    }
}

// shutdown Chip8
fn finalize() {
    println!("Shutdown");
}

// Main entry point
fn main() {
    let args = Args::parse();

    println!("Program file: {}", args.program);
    println!("Using screen size {}x{}", args.screen_size_x, args.screen_size_y);

    let mut components = Chip8Components::default();
    let mut memory = Memory::new(args.memory_size);
    // let event_loop = ActiveEventLoop::new();
    let mut display = Display::new(args.screen_size_x, args.screen_size_y);

    initialize(&mut components, &mut memory, &args);
    main_loop(&mut components, &mut memory, &mut display);
    // finalize();
}
