#![allow(arithmetic_overflow)]

use core::time;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::thread::sleep;

use crate::display::Display;
use crate::cpu::chip8_components::Chip8Components;
use crate::memory::Memory;
use crate::launch_arguments::Args;
use crate::cpu::opcodes::{ArithmeticCode, NibbleCode};
use rand::{Rng, RngExt};

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
fn clear_screen(display: &mut Display) {
    // println!("Clearing screen");
    display.clear_buffer();
}

// Sets the program counter to an address.
fn jump(components: &mut Chip8Components, address: u16) {
    // println!("Jumping to {:x}", address);
    components.program_counter = address;
}

// skips one instruction if the value in vx is equal to nn.
fn branch_eq(components: &mut Chip8Components, x: u8, nn: u8) {
    let vx_value = components.registers[x as usize];
    if vx_value == nn {
        components.program_counter += 2;
    }
}

fn branch_ne(components: &mut Chip8Components, x: u8, nn: u8) {
    let vx_value: u8 = components.registers[x as usize];
    if vx_value != nn {
        components.program_counter += 2;
    }
}

fn branch_req(components: &mut Chip8Components, x: u8, y: u8) {
    let vx_value = components.registers[x as usize];
    let vy_value: u8 = components.registers[y as usize];
    if vx_value == vy_value {
        components.program_counter += 2;
    }
}

// Set the register X to NN.
 fn set_register(components: &mut Chip8Components, register: u8, value: u8) {
    // println!("Setting V{:x} to {:x}", register, value);
    components.registers[register as usize] = value;
}

// Add a value to register X. Does not set the carry flag if the result overflows.
fn add_register(components: &mut Chip8Components, register: u8, value: u8) {
    // println!("Adding {:x} to V{:x}", value, register);
    components.registers[register as usize] += value;
}

fn arith_set(components: &mut Chip8Components, x: u8, y: u8) {
    components.registers[x as usize] = components.registers[y as usize];
}

fn arith_or(components: &mut Chip8Components, x: u8, y: u8) {
    let x_value = components.registers[x as usize];
    let y_value: u8 = components.registers[y as usize];
    components.registers[x as usize] = x_value | y_value;
}

fn arith_and(components: &mut Chip8Components, x: u8, y: u8) {
    let x_value = components.registers[x as usize];
    let y_value: u8 = components.registers[y as usize];
    components.registers[x as usize] = x_value & y_value;
}

fn arith_xor(components: &mut Chip8Components, x: u8, y: u8) {
    let x_value = components.registers[x as usize];
    let y_value: u8 = components.registers[y as usize];
    components.registers[x as usize] = x_value ^ y_value;
}

fn arith_add(components: &mut Chip8Components, x: u8, y: u8) {
    let x_value: u8 = components.registers[x as usize];
    let y_value: u8 = components.registers[y as usize];
    components.registers[x as usize] = x_value + y_value;
    components.registers[0xF] = if x_value as u16 + y_value as u16 > 255 { 1 } else { 0 };
}

fn arith_subtract_xy(components: &mut Chip8Components, x: u8, y: u8) {
    let x_value: u8 = components.registers[x as usize];
    let y_value: u8 = components.registers[y as usize];
    components.registers[x as usize] = x_value - y_value;
    components.registers[0xF] = if x_value >= y_value { 1 } else { 0 };
}

fn arith_subtract_yx(components: &mut Chip8Components, x: u8, y: u8) {
    let x_value: u8 = components.registers[x as usize];
    let y_value: u8 = components.registers[y as usize];
    components.registers[x as usize] = y_value - x_value;
    components.registers[0xF] = if y_value >= x_value { 1 } else { 0 };
}

fn arith_shift_right(components: &mut Chip8Components, vx: u8, vy: u8) {
    // todo:: add flag for newer chip8 implementations
    let x: u8 = components.registers[vx as usize];
    let y: u8 = components.registers[vy as usize];
    let mut shifted_out: u8 = 0;

    // original behavior (vy moves into vx)
    if true {
        shifted_out = y & 0x1;
        components.registers[vx as usize] = y >> 1;
    }

    // todo:: new behavior (shift in-place and ignore vy)
    if false {
        shifted_out = x & 0x1;
        components.registers[vx as usize] = x >> 1;
    }
    
    // set flag register
    components.registers[0xF as usize] = shifted_out;
}

fn arith_shift_left(components: &mut Chip8Components, vx: u8, vy: u8) {
    // todo:: add flag for newer chip8 implementations
    // todo:: add flag for newer chip8 implementations
    let x: u8 = components.registers[vx as usize];
    let y: u8 = components.registers[vy as usize];
    let mut shifted_out: u8 = 0;

    // original behavior (vy moves into vx)
    if true {
        shifted_out = y & 0x40;
        components.registers[vx as usize] = y << 1;
    }

    // todo:: new behavior (shift in-place and ignore vy)
    if false {
        shifted_out = x & 0x40;
        components.registers[vx as usize] = x << 1;
    }
    
    // set flag register
    components.registers[0xF as usize] = shifted_out;
}

fn branch_rne(components: &mut Chip8Components, x: u8, y: u8) {
    let vx_value = components.registers[x as usize];
    let vy_value: u8 = components.registers[y as usize];
    if vx_value != vy_value {
        components.program_counter += 2;
    }
}

// Set the index register to an address.
fn set_index_register(components: &mut Chip8Components, address: u16) {
    // println!("Set index register to {:x}", address);
    components.index_register = address;
}

// jump to address plus value in register
fn jump_with_offset(components: &mut Chip8Components, vx: u8, address: u16) {
    // todo:: new behavior jumps to XNN with offset of VX, old uses V0
    let offset: u8 = if true { components.registers[0x0 as usize] } else { components.registers[vx as usize] };
    components.program_counter = address + offset as u16;
}

fn random(components: &mut Chip8Components, x: u8, nn: u8) {
    let mut rng = rand::rng();
    let value = rng.random::<u8>() & nn;
    components.registers[x as usize] = value;
}

// draw a sprite at x and y location from the memory location that the index register
// points to.
fn draw(components: &mut Chip8Components, memory: &mut Memory, display: &mut Display, x: u8, y: u8, rows: u8) {
    // get x and y coordinates
    let x_loc: u8 = components.registers[x as usize];
    let y_loc: u8 = components.registers[y as usize];

    // set register F to 0 (flag register)
    components.registers[0xF as usize] = 0;

    // for n rows (height of the sprite in bytes)
    for row in 0..rows {
        // get nth byte of sprite data starting from index register
        let sprite_byte: u8 = memory.read(components.index_register + row as u16);

        // for each of the 8 bits in the this sprite row
        for col in 0..8 {
            // not entirely sure what this is doing (we're checking each bit of the sprite byte)
            if (sprite_byte & (0x80 >> col)) != 0 {
                // set the x and y coordinates using modulo with the screen size
                let target_x_loc = (x_loc + col) % 64;
                let target_y_loc = (y_loc + row) % 32;
                let result = display.read_buffer(target_x_loc as u32, target_y_loc as u32) ^ true;

                // if current pixel in row is on and pixel at x,y is on, turn off pixel, set register F to 1
                if result == true {
                    components.registers[0xF as usize] = 1;
                }

                display.write_buffer(target_x_loc as u32, target_y_loc as u32, result);
            }

            // incrememnt X (not VX)
            // x_loc += 1;
        }
        // increment Y
        // y_loc += 1;
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

fn execute(data: &DecodeData, components: &mut Chip8Components, memory: &mut Memory, display: &mut Display) {

    match data.opcode {
        // 0x0
        val if val == NibbleCode::Misc as u8 => {
            if data.second_byte == 0xE0 {
                clear_screen(display);
            }

            if data.second_byte == 0xEE {
                panic!("Subroutines not implemented");
            }
        },
        
        // 0x1
        val if val == NibbleCode::Jump as u8 => {
            jump(components, data.immediate_mem_address);
        },

        // 0x2
        val if val == NibbleCode::Subroutines as u8 => {
            panic!("Subroutines not implemented {}", val);
        },

        // 0x3
        val if val == NibbleCode::BranchEqual as u8 => {
            branch_eq(components, data.register_x, data.second_byte);
        },

        // 0x4
        val if val == NibbleCode::BranchNotEqual as u8 => {
            branch_ne(components, data.register_x, data.second_byte);
        },

        // 0x5
        val if val == NibbleCode::BranchRegisterEqual as u8 => {
            branch_req(components, data.register_x, data.register_y);
        },

        // 0x6
        val if val == NibbleCode::SetRegister as u8 => {
            set_register(components, data.register_x, data.second_byte);
        },

        // 0x7
        val if val == NibbleCode::AddToRegister as u8 => {
            add_register(components, data.register_x, data.second_byte);
        },

        // 0x8
        val if val == NibbleCode::Arithmetic as u8 => {
            match data.constant {
                val if val == ArithmeticCode::Set as u8 => {
                    arith_set(components, data.register_x, data.register_y);
                },

                val if val == ArithmeticCode::Or as u8 => {
                    arith_or(components, data.register_x, data.register_y);
                },

                val if val == ArithmeticCode::And as u8 => {
                    arith_and(components, data.register_x, data.register_y);
                },

                val if val == ArithmeticCode::Xor as u8 => {
                    arith_xor(components, data.register_x, data.register_y);
                },

                val if val == ArithmeticCode::Add as u8 => {
                    arith_add(components, data.register_x, data.register_y);
                },

                val if val == ArithmeticCode::SubtractXy as u8 => {
                    arith_subtract_xy(components, data.register_x, data.register_y);
                },

                val if val == ArithmeticCode::SubtractYx as u8 => {
                    arith_subtract_yx(components, data.register_x, data.register_y);
                },

                val if val == ArithmeticCode::ShiftRight as u8 => {
                    arith_shift_right(components, data.register_x, data.register_y);
                },

                val if val == ArithmeticCode::ShiftLeft as u8 => {
                    arith_shift_left(components, data.register_x, data.register_y);
                },

                _ => { panic!("Unimplemented arithmetic instruction"); }
            }
        },

        // 0x9
        val if val == NibbleCode::BranchRegisterNotEqual as u8 => {
            branch_rne(components, data.register_x, data.register_y);
        },

        // 0xA
        val if val == NibbleCode::SetIndexRegister as u8 => {
            set_index_register(components, data.immediate_mem_address);
        },

        // 0xB
        val if val == NibbleCode::JumpWithOffset as u8 => {
            jump_with_offset(components, data.register_x, data.immediate_mem_address);
        },

        // 0xC
        val if val == NibbleCode::Random as u8 => {
            random(components, data.register_x, data.second_byte);
        }

        // 0xD
        val if val == NibbleCode::Draw as u8 => {
            draw(components, memory, display, data.register_x, data.register_y, data.constant);
        },

        // 0xE
        val if val == NibbleCode::SkipKey as u8 => {
            panic!("SkipKey not implemented");
        },

        // 0xF
        val if val == NibbleCode::Timer as u8 => {
            panic!("Timer not implemented");
        },
        
        _ => { 
            panic!("Unimplemented Opcode");
        },
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
    execute(&instruction, components, memory, display);
}

// update the frame, including graphics, audio, etc
fn frame_update_step(components: &mut Chip8Components, display: &mut Display) {
    display.update();
}

// Initialize the CPU.
fn initialize(components: &mut Chip8Components, memory: &mut Memory, display: &mut Display, args: &Args) -> i32 {
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
fn main_loop(components: &mut Chip8Components, memory: &mut Memory, display: &mut Display, fps: u32, ips: u32) -> i32 {
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