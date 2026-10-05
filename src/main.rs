use clap::Parser;
use std::fs;
use std::io;
use winit::dpi::LogicalSize;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::Window;

use crate::nibble_codes::NibbleCode;
use crate::instructions::clear_screen;
use crate::memory::Memory;

struct DecodeData {
    immediate_mem_address: u16,
    operation: u16,
    first_byte: u16,
    second_byte: u16,
    register_x: u8,
    register_y: u8,
    constant: u16,
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

fn decode(bytes: u16) -> DecodeData {
    println!("Got instruction {:x}", bytes);
    let first_byte = (bytes & 0xFF00) >> 8;
    let second_byte = bytes & 0x00FF; // contains 3rd and 4th nibbles, can be used as an immediate number
    let immediate_mem_address = bytes & 0x0FFF; // 2md, 3rd, 4th nibbles, may be used for a 12 bit memory address

    // split up nibbles
    let operation = (bytes & 0xF000) >> 12;
    let register_x = (bytes & 0x0F00) >> 8;
    let register_y = (bytes & 0x00F0) >> 4;
    let constant = bytes & 0x000F;

    DecodeData {
        immediate_mem_address: immediate_mem_address,
        operation: operation,
        first_byte: first_byte,

    }
}

fn execute(data: &DecodeData) {

    match operation {
        NibbleCode::ClearScreen => {
            
        },
        
        NibbleCode::Jump => {
            
        },

        NibbleCode::SetRegister => {
            
        }

        NibbleCodes::AddToRegister => {
            
        },

        NibbleCodes::SetIndexRegister => {
            
        },

        NibbleCodes::Draw => {
            
        },
        
        _ => { 
            println!("Unknown instruction {:x}", bytes)
        }
    }
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

        let instruction  = decode(instruction_bytes);
        execute(&instruction);
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
    finalize();
}
