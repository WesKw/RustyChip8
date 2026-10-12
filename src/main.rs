use clap::Parser;

mod cpu;
mod display;
mod launch_arguments;
mod memory;
mod logger;

use crate::cpu::core;
use crate::launch_arguments::Args;
use crate::logger::log_functions;
use crate::logger::log_levels::LogLevel;

// Main entry point
fn main() {
    let args = Args::parse();

    log_functions::set_log_level(LogLevel::try_from(args.log_level).unwrap());
    log_functions::print_cli_args(&args);

    // begin main loop
    let rc = core::start(&args);

    // print return code when exiting
    log_functions::print_return_code(rc);
}
