use clap::Parser;
use std::fs;
use std::io;
use winit::dpi::LogicalSize;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::Window;
use std::time::{SystemTime, UNIX_EPOCH};

mod nibble_codes;
mod launch_arguments;
mod memory;
mod display;
mod chip8_components;
mod instructions;

use crate::nibble_codes::NibbleCode;
use crate::instructions::clear_screen;
use crate::memory::Memory;
use crate::chip8_components::Chip8Components;
use crate::launch_arguments::Args;
use crate::display::Display;

struct DecodeData {
    immediate_mem_address: u16,
    opcode: u16,
    first_byte: u16,
    second_byte: u16,
    register_x: u16,
    register_y: u16,
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
        opcode: operation,
        first_byte: first_byte,
        second_byte: second_byte,
        register_x: register_x,
        register_y: register_y,
        constant: constant,
    }
}

fn execute(data: &DecodeData, components: &mut Chip8Components, display: &mut Display) {

    match data.opcode {
        val if val == NibbleCode::ClearScreen as u16 => {
            println!("Clear screen");
            // instructions::clear_screen(display);
        },
        
        val if val == NibbleCode::Jump as u16 => {
            
        },

        val if val == NibbleCode::SetRegister as u16 => {
            
        }

        val if val == NibbleCode::AddToRegister as u16 => {
            
        },

        val if val == NibbleCode::SetIndexRegister as u16 => {
            
        },

        val if val == NibbleCode::Draw as u16 => {
            
        },
        
        _ => { 
            println!("Unknown opcode {:x}", data.opcode);
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
    let cycle_duration: f64 = 1.0 / 700.0; // chip8 has 700 hz clock
    let update_rate: f64 = 1.0 / 60.0; // screen refresh rate;
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Bad time");
    let time_f64 = time.as_secs() as f64 + time.subsec_nanos() as f64;
    let mut next_cycle: f64 = time_f64 + cycle_duration;
    let mut next_screen_update: f64 = time_f64 + update_rate;

    loop {
        // get current time
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Bad time");
        let current_time_f64 = current_time.as_secs() as f64 + current_time.subsec_nanos() as f64;

        if current_time_f64 >= next_cycle {
            println!("Next instruction");

            let instruction_bytes: u16 = fetch(components, memory);

            // temporary break if we hit an empty instruction
            if instruction_bytes == 0 {
                break;
            }

            let instruction  = decode(instruction_bytes);
            execute(&instruction, components, display);

            // update for next cycle
            next_cycle = current_time_f64 + cycle_duration;
        }

        if current_time_f64 >= next_screen_update {
            println!("Update screen");

            // update the screen
            // display.update();

            next_screen_update = current_time_f64 + update_rate;
        }
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
