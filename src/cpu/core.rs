#![allow(arithmetic_overflow)]

use core::time;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::thread::sleep;

use crate::display::Display;
use crate::cpu::chip8_components::Chip8Components;
use crate::memory::Memory;
use crate::launch_arguments::Args;
use crate::cpu::opcodes::NibbleCode;

pub struct DecodeData {
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
    // println!("Clearing screen");
    display.clear_buffer();
}

// Sets the program counter to an address.
pub fn jump(components: &mut Chip8Components, address: u16) {
    // println!("Jumping to {:x}", address);
    components.program_counter = address;
}

// Set the register X to NN.
pub fn set_register(components: &mut Chip8Components, register: u8, value: u8) {
    // println!("Setting V{:x} to {:x}", register, value);
    components.registers[register as usize] = value;
}

// Add a value to register X. Does not set the carry flag if the result overflows.
pub fn add_register(components: &mut Chip8Components, register: u8, value: u8) {
    // println!("Adding {:x} to V{:x}", value, register);
    components.registers[register as usize] += value;
}

// Set the index register to an address.
pub fn set_index_register(components: &mut Chip8Components, address: u16) {
    // println!("Set index register to {:x}", address);
    components.index_register = address;
}

// draw a sprite at x and y location from the memory location that the index register
// points to.
pub fn draw(components: &mut Chip8Components, display: &mut Display, x: u8, y: u8, rows: u8) {
    // get x and y coordinates
    let mut x_loc: u8 = components.registers[x as usize];
    let mut y_loc: u8 = components.registers[y as usize];

    // set the x and y coordinates using modulo with the screen size
    x_loc = x_loc & 63;
    y_loc = y_loc & 31;

    // set register F to 0 (flag register)
    components.registers[0xF as usize] = 0;

    // for n rows (height of the sprite in bytes)
    for row in 0..rows {
        // get nth byte of sprite data starting from index register

        // for each of the 8 bits in the this sprite row
        for ... {
            // if current pixel in row is on and pixel at x,y is on, turn off pixel, set register F to 1

            // or if the pixel in the spirte row and the screen pixel isn't draw pixel at x,y

            // if the edge of the screen is reached, stop drawing the row

            // incrememnt X (not VX)
            x_loc += 1;
        }

        // increment Y
        y_loc += 1;
    }

    // display.write_buffer(x_loc as u32, y_loc as u32, value as u32);
}


fn fetch(components: &mut Chip8Components, memory: &Memory) -> u16 {
    if components.program_counter as usize >= memory.mem_size() {
        println!("Hit end of program");
        return 0x0
    }

    // get the next instruction from memory
    // instruction is 2 bytes
    // println!("Reading instruction at PC: {:x}", components.program_counter);
    let byte1: u16 = (memory.read(components.program_counter) as u16) << 8; // shift over one byte
    let byte2: u16 = memory.read(components.program_counter + 1) as u16;
    let insn: u16 = byte1 | byte2;

    // update program counter
    components.program_counter += 2;

    // return the insn
    insn
}   

fn decode(bytes: u16) -> DecodeData {
    // println!("Got instruction {:x}", bytes);
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
            draw(components, display, data.register_x, data.register_y, data.constant);
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

// Initialize the CPU.
pub fn initialize(components: &mut Chip8Components, memory: &mut Memory, display: &mut Display, args: &Args) -> i32 {
    // load default components of Chip8
    println!("Initializing Chip8");

    // load program into memory
    let prog: Vec<u8> = fs::read(&args.program).expect("Failed to read program file");
    let start: u16 = 0x200; // typical start address for Chip8 programs
    println!("start {}", start);
    for (i, &byte) in prog.iter().enumerate() {
        memory.write(start + i as u16, byte);
    }

    // set PC to first instruction in memory
    components.program_counter = start;

    // generate window
    display.update(); // initialize the display

    0
}

// shutdown Chip8
pub fn finalize() -> i32 {
    println!("Shutdown");
    0
}

// Fetch/decode/execute loop
pub fn main_loop(components: &mut Chip8Components, memory: &mut Memory, display: &mut Display, fps: u32, ips: u32) -> i32 {
    let update_rate: f64 = 1.0 / fps as f64; // screen refresh rate;

    loop {
        let start = Instant::now();
        // run n instructions per frame
        for _i in 1..=ips {
            input_step(components, memory);
            cpu_step(components, memory, display);
        }

        frame_update_step(components, display);

        // then wait until we need to perform the next update
        let elapsed = Instant::now() - start;
        let sleep_time = time::Duration::from_secs_f64(update_rate) - elapsed;
        sleep(sleep_time);
        // println!("Slept for {:?}", sleep_time);
    }

    0
}


pub fn start(args: &Args) -> i32 {
    let mut components = Chip8Components::default();
    let mut memory = Memory::new(args.memory_size);
    memory.setup_font_data();
    // let event_loop = ActiveEventLoop::new();
    let mut display = Display::new(args.screen_size_x, args.screen_size_y);
    
    let mut rc = initialize(&mut components, &mut memory, &mut display, &args);
    if rc != 0 { return rc; }

    rc = main_loop(&mut components, &mut memory, &mut display, args.frames_per_second, args.instructions_per_frame);
    if rc != 0 { return rc; }

    rc = finalize();
    rc
}