use crate::chip8_components::Chip8Components;
use crate::memory::Memory;
use crate::display::Display;

// executes a machine leanguage routine, pausing program and calling a subroutine
// written in asm.
// pub fn execute_ml_routine() {

// }

// Clears the screen.
pub fn clear_screen(display: &mut Display) {
    display.clear_buffer();
    display.update();
}

// Sets the program counter to an address.
pub fn jump(components: &mut Chip8Components, address: u16) {
    components.program_counter = address;
}

// Set the register X to NN.
pub fn set_register(components: &mut Chip8Components, register: u8, value: u16) {

}

// Add a value to register X.
pub fn add_register(components: &mut Chip8Components, register: u8, value: u16) {

}

// Set the index register to an address.
pub fn set_index_register(components: &mut Chip8Components, address: u16) {

}

pub fn draw(display: &mut Display, x: u8, y: u8, value: u8) {
    
}

// pub fn 