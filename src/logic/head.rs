use csv::ReaderBuilder;

use crate::{enums::commands::CommandType, structs::config::Config};

pub fn print_dataframe(config: &Config) {
    let file = config.input_file.as_ref().unwrap();

    let mut rdr = ReaderBuilder::new()
        .delimiter(config.separator)
        .has_headers(config.has_header)
        .from_path(file)
        .unwrap();

    let header = rdr.headers();
    if let Ok(row) = header {
        match config.pretty {
            true => print_clean(row, true),
            false => print_normal(row, true),
        }
    }

    match config.command_type {
        CommandType::Head => print_head(config, &mut rdr),
        CommandType::Tail => print_tail(config, &mut rdr),
        _ => panic!("Invalid command type"),
    }
}

fn print_head(config: &Config, rdr: &mut csv::Reader<std::fs::File>) {
    let records = rdr.records().take(config.num_rows.unwrap());
    for record in records {
        match config.pretty {
            true => print_clean(&record.unwrap(), false),
            false => print_normal(&record.unwrap(), false),
        }
    }
}

fn print_tail(config: &Config, rdr: &mut csv::Reader<std::fs::File>) {
    let num_rows = rdr.records().count();
    let mut rows_to_show = config.num_rows.unwrap();
    let mut rows_to_skip = 0;

    if rows_to_show > num_rows {
        rows_to_show = num_rows;
    } else {
        rows_to_skip = num_rows - rows_to_show;
    }

    let records = rdr.records().skip(rows_to_skip).take(rows_to_show);
    for record in records {
        match config.pretty {
            true => print_clean(&record.unwrap(), false),
            false => print_normal(&record.unwrap(), false),
        }
    }
}

fn print_normal(record: &csv::StringRecord, underline: bool) {
    for field in record.iter() {
        print!("{:?},", field);
    }
    println!();
}

fn print_clean(record: &csv::StringRecord, underline: bool) {
    let mut s = String::new();
    for field in record.iter() {
        s += format!("{:<8}| ", field).as_str();
    }
    s = s.trim_end_matches(", ").to_string();
    println!("{}", s);
    if underline {
        for _ in record.iter() {
            print!("--------|");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_normal() {
        let args = vec![
            "my_program".to_string(),
            "-f".to_string(),
            "3".to_string(),
            "testfile/wine.csv".to_string(),
        ];
        let config = Config::new(&args);

        print_dataframe(&config);
    }

    #[test]
    fn test_head_pretty() {
        let args = vec![
            "my_program".to_string(),
            "-f".to_string(),
            "3".to_string(),
            "--pretty".to_string(),
            "testfile/wine.csv".to_string(),
        ];
        let config = Config::new(&args);

        print_dataframe(&config);
    }

    #[test]
    fn test_head_invalid_separator() {
        let args = vec![
            "my_program".to_string(),
            "-f".to_string(),
            "10".to_string(),
            "-s".to_string(),
            ";".to_string(),
            "testfile/wine.csv".to_string(),
        ];
        let config = Config::new(&args);

        print_dataframe(&config);
    }
}
