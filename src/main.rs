extern crate getopts;
use getopts::Options;
use std::env;

mod enums;
mod structs;

use enums::commands::CommandType;
use structs::config::Config;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("{:?}", config);

    match config.command_type {
        CommandType::Help => {
            print_usage(&args[0], Config::get_opts());
        }
        CommandType::Head => {}
        CommandType::Tail => {}
        CommandType::Column => {}
    }
}
