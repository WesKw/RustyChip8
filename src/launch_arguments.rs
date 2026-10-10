use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub program: String, // program file path

    #[arg(long, default_value_t=4096)]
    pub memory_size: u16, // total memory capacity

    #[arg(long, default_value_t=64)]
    pub screen_size_x: u32, // horizontal screen size
    #[arg(long, default_value_t=32)]
    pub screen_size_y: u32, // vertical screen size

    // #[arg(short, long, default_value_t=1)]
    // clock_speed_mhz: u8
    #[arg(long, default_value_t=11)]
    pub instructions_per_frame: u32, // instructions to run perframe (default is 11 -> 60 * 11 = 660 instructions per second)

    #[arg(long, default_value_t=60)]
    pub frames_per_second: u32, // number of screen updates per second (default is 60)

    #[arg(long, default_value_t=0)]
    pub log_level: i32,
}