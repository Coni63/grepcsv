use getopts::Options;

use crate::enums::commands::CommandType;

#[derive(Debug)]
pub struct Config {
    pub program: String,
    pub input_file: Option<String>,

    pub command_type: CommandType,
    pub num_rows: Option<i32>,
    pub column_name: Option<String>,
    pub column_index: Option<i32>,
}

impl Config {
    pub fn get_opts() -> Options {
        let mut opts = Options::new();
        opts.optopt("f", "first", "show the N first lines", "10");
        opts.optopt("l", "last", "show the N last lines", "10");
        opts.optopt("n", "column-name", "show the column by name", "column_A");
        opts.optopt("i", "column-index", "show the column by index", "4");
        opts.optflag("h", "help", "print this help menu");
        opts
    }

    pub fn new(args: &[String]) -> Config {
        let program = args[0].clone();

        let matches = match Config::get_opts().parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                panic!("Error while parsing command: {}", f);
            }
        };

        let input_file = matches.free.first().cloned();
        let mut command_type = CommandType::Head;
        let mut num_rows = None;
        let mut column_name = None;
        let mut column_index = None;

        if matches.opt_present("h") {
            command_type = CommandType::Help;
        } else if matches.opt_present("f") {
            command_type = CommandType::Head;
            num_rows = matches.opt_str("f").map(|x| x.parse().unwrap());
        } else if matches.opt_present("l") {
            command_type = CommandType::Tail;
            num_rows = matches.opt_str("l").map(|x| x.parse().unwrap());
        } else if matches.opt_present("n") {
            command_type = CommandType::Column;
            column_name = matches.opt_str("n");
        } else if matches.opt_present("i") {
            command_type = CommandType::Column;
            column_index = matches.opt_str("i").map(|x| x.parse().unwrap());
        }

        Config {
            program,
            input_file,
            command_type,
            num_rows,
            column_name,
            column_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_short() {
        let args = vec!["my_program".to_string(), "-h".to_string()];
        let config = Config::new(&args);

        assert_eq!(config.program, "my_program");
        assert_eq!(config.input_file, None);
        assert_eq!(config.command_type, CommandType::Help);
    }

    #[test]
    fn test_help_long() {
        let args = vec!["my_program".to_string(), "--help".to_string()];
        let config = Config::new(&args);

        assert_eq!(config.program, "my_program");
        assert_eq!(config.input_file, None);
        assert_eq!(config.command_type, CommandType::Help);
    }
    #[test]
    fn test_help_complete() {
        let args = vec![
            "my_program".to_string(),
            "--help".to_string(),
            "-f".to_string(),
            "10".to_string(),
            "filename".to_string(),
        ];
        let config = Config::new(&args);

        assert_eq!(config.program, "my_program");
        assert_eq!(config.input_file.unwrap(), "filename".to_string());
        assert_eq!(config.command_type, CommandType::Help);
    }

    #[test]
    fn test_head_short() {
        let args = vec![
            "my_program".to_string(),
            "-f".to_string(),
            "10".to_string(),
            "filename".to_string(),
        ];
        let config = Config::new(&args);

        assert_eq!(config.program, "my_program");
        assert_eq!(config.input_file.unwrap(), "filename".to_string());
        assert_eq!(config.command_type, CommandType::Head);
        assert_eq!(config.num_rows.unwrap(), 10);
    }

    #[test]
    fn test_tail_short() {
        let args = vec![
            "my_program".to_string(),
            "-l".to_string(),
            "10".to_string(),
            "filename".to_string(),
        ];
        let config = Config::new(&args);

        assert_eq!(config.program, "my_program");
        assert_eq!(config.input_file.unwrap(), "filename".to_string());
        assert_eq!(config.command_type, CommandType::Tail);
        assert_eq!(config.num_rows.unwrap(), 10);
    }

    #[test]
    fn test_column_name_short() {
        let args = vec![
            "my_program".to_string(),
            "-n".to_string(),
            "column_A".to_string(),
            "filename".to_string(),
        ];
        let config = Config::new(&args);

        assert_eq!(config.program, "my_program");
        assert_eq!(config.input_file.unwrap(), "filename".to_string());
        assert_eq!(config.command_type, CommandType::Column);
        assert_eq!(config.column_name.unwrap(), "column_A".to_string());
    }

    #[test]
    fn test_column_index_short() {
        let args = vec![
            "my_program".to_string(),
            "-i".to_string(),
            "4".to_string(),
            "filename".to_string(),
        ];
        let config = Config::new(&args);

        assert_eq!(config.program, "my_program");
        assert_eq!(config.input_file.unwrap(), "filename".to_string());
        assert_eq!(config.command_type, CommandType::Column);
        assert_eq!(config.column_index.unwrap(), 4);
    }
}
