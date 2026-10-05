use clap::Parser;

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
    instructions_per_second: u32, // instructions per second
}