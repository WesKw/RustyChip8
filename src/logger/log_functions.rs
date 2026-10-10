use crate::logger::return_codes::ReturnCode;
use crate::logger::log_levels::LogLevel;
use crate::launch_arguments::Args;
use std::sync::{OnceLock};

static LOG_LEVEL: OnceLock<LogLevel> = OnceLock::new();

pub fn set_log_level(level: LogLevel) {
    println!("TODO:: Log levels not implemented");
    LOG_LEVEL.set(level).expect("Static variable was already initialized!");
}

pub fn print_cli_args(args: &Args) {
    // if LOG_LEVEL.get().unwrap() != LogLevel::None {
    println!("Program file: {}", args.program);
    println!("Using screen size {}x{}", args.screen_size_x, args.screen_size_y);
    println!("Frames-per-second: {}", args.frames_per_second);
    println!("Instructions-per-frame: {}", args.instructions_per_frame);
    println!("Using log-level {}", args.log_level);
    // }
}

pub fn print_return_code(rc: i32) {
    match rc {
        val if val == ReturnCode::Success as i32 => println!("Successful execution."),
        val if val == ReturnCode::Failure as i32 => println!("Something went wrong."),
        _ => println!("Unknown failure code {}", rc)
    }
}