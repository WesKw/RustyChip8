use clap::Parser;

mod launch_arguments;
mod memory;
mod display;
mod cpu;

use crate::cpu::{start};
use crate::memory::Memory;
use crate::cpu::chip8_components::Chip8Components;
use crate::launch_arguments::Args;
use crate::display::Display;

// Main entry point
fn main() {
    let args = Args::parse();

    println!("Program file: {}", args.program);
    println!("Using screen size {}x{}", args.screen_size_x, args.screen_size_y);

    let mut components = Chip8Components::default();
    let mut memory = Memory::new(args.memory_size);
    // let event_loop = ActiveEventLoop::new();
    let mut display = Display::new(args.screen_size_x, args.screen_size_y);

    display.update(); // initialize the display

    cpu::start();

    initialize(&mut components, &mut memory, &args);
    main_loop(&mut components, &mut memory, &mut display, &args);
    finalize();
}
