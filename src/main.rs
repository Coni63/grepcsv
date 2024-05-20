use std::env;

mod enums;
mod logic;
mod structs;

use enums::commands::CommandType;
use logic::{print_column, print_dataframe, print_usage};
use structs::config::Config;

fn check_file_exists(config: &Config) {
    if let Some(file) = &config.input_file {
        if !std::path::Path::new(file).exists() {
            panic!("File does not exist: {}", file);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    check_file_exists(&config);

    println!("{:?}", config);

    match config.command_type {
        CommandType::Help => {
            print_usage(&config);
        }
        CommandType::Head => {
            print_dataframe(&config);
        }
        CommandType::Tail => {
            print_dataframe(&config);
        }
        CommandType::ColumnName => {
            print_column(&config);
        }
        CommandType::ColumnIndex => {
            print_column(&config);
        }
    }
}
