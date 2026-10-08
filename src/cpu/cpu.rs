#![allow(arithmetic_overflow)]

use core::time;
use std::fs;
use std::io;
use winit::dpi::LogicalSize;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::Window;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::thread::sleep;

use crate::chip8_components::Chip8Components;
use crate::memory::Memory;
use crate::display::Display;

struct DecodeData {
    immediate_mem_address: u16,
    opcode: u8,
    first_byte: u8,
    second_byte: u8,
    register_x: u8,
    register_y: u8,
    constant: u8,
}

// Clears the screen.
pub fn clear_screen(display: &mut Display) {
    println!("Clearing screen");
    display.clear_buffer();
}

// Sets the program counter to an address.
pub fn jump(components: &mut Chip8Components, address: u16) {
    println!("Jumping to {:x}", address);
    components.program_counter = address;
}

// Set the register X to NN.
pub fn set_register(components: &mut Chip8Components, register: u8, value: u8) {
    println!("Setting V{:x} to {:x}", register, value);
    components.registers[register as usize] = value;
}

// Add a value to register X. Does not set the carry flag if the result overflows.
pub fn add_register(components: &mut Chip8Components, register: u8, value: u8) {
    println!("Adding {:x} to V{:x}", value, register);
    components.registers[register as usize] += value;
}

// Set the index register to an address.
pub fn set_index_register(components: &mut Chip8Components, address: u16) {
    println!("Set index register to {:x}", address);
    components.index_register = address;
}

pub fn draw(display: &mut Display, x: u8, y: u8, value: u8) {
    println!("Writing to display buffer");
    display.write_buffer(x as u32, y as u32, value as u32);
}


fn fetch(components: &mut Chip8Components, memory: &Memory) -> u16 {
    if components.program_counter as usize >= memory.mem_size() {
        println!("Hit end of program");
        return 0x0
    }

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
    let first_byte: u8 = ((bytes & 0xFF00) >> 8).try_into().unwrap();
    let second_byte: u8 = ((bytes & 0x00FF) as u8).try_into().unwrap(); // contains 3rd and 4th nibbles, can be used as an immediate number
    let immediate_mem_address = bytes & 0x0FFF; // 2md, 3rd, 4th nibbles, may be used for a 12 bit memory address

    // split up nibbles
    let operation: u8 = ((bytes & 0xF000) >> 12).try_into().unwrap();
    let register_x: u8 = ((bytes & 0x0F00) >> 8).try_into().unwrap();
    let register_y: u8 = ((bytes & 0x00F0) >> 4).try_into().unwrap();
    let constant: u8 = (bytes & 0x000F).try_into().unwrap();

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
        val if val == NibbleCode::ClearScreen as u8 => {
            clear_screen(display);
        },
        
        val if val == NibbleCode::Jump as u8 => {
            jump(components, data.immediate_mem_address);
        },

        val if val == NibbleCode::SetRegister as u8 => {
            set_register(components, data.register_x, data.second_byte);
        }

        val if val == NibbleCode::AddToRegister as u8 => {
            add_register(components, data.register_x, data.second_byte);
        },

        val if val == NibbleCode::SetIndexRegister as u8 => {
            set_index_register(components, data.immediate_mem_address);
        },

        val if val == NibbleCode::Draw as u8 => {
            draw(display, data.register_x as u8, data.register_y, data.constant);
        },
        
        _ => { 
            panic!("Unknown Opcode");
        }
    }
}

// gather input from user
fn input_step(components: &mut Chip8Components, memory: &mut Memory) {

}

// run cpu instructions
fn cpu_step(components: &mut Chip8Components, memory: &mut Memory, display: &mut Display) {
    // fetch next instruction
    let instruction_bytes: u16 = fetch(components, memory);
    // decode the fetched instruction
    let instruction = decode(instruction_bytes);
    //execute instruction based on decode
    execute(&instruction, components, display);
}

// update the frame, including graphics, audio, etc
fn frame_update_step(components: &mut Chip8Components, display: &mut Display) {
    display.update();
}

pub fn start() {
    
}

// Initialize the CPU.
pub fn initialize(components: &mut Chip8Components, memory: &mut Memory, args: &Args) {
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

// shutdown Chip8
pub fn finalize() {
    println!("Shutdown");
}

// Fetch/decode/execute loop
pub fn main_loop(components: &mut Chip8Components, memory: &mut Memory, display: &mut Display, args: &Args) {
    let update_rate: f64 = 1.0 / args.frames_per_second as f64; // screen refresh rate;
    // let mut cpu_clock = Instant::now().elapsed().as_secs_f64();
    // println!("{}", cpu_clock);
    // let mut next_cycle: f64 = cpu_clock + cycle_duration;
    // let mut next_screen_update: f64 = time_seconds + update_rate;
    // println!("{}", next_cycle);
    // println!("{}", next_screen_update);

    loop {
        let start = Instant::now();
        // run n instructions per frame
        for _i in 1..=args.instructions_per_frame {
            input_step(components, memory);
            cpu_step(components, memory, display);
        }

        frame_update_step(components, display);

        // then wait until we need to perform the next update
        let elapsed = Instant::now() - start;
        sleep(time::Duration::from_secs_f64(update_rate) - elapsed);
    }
}

// pub fn 