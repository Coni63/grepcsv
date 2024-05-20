use csv::{Position, ReaderBuilder, StringRecord};
use std::cmp::max;

use crate::{enums::commands::CommandType, structs::config::Config};

pub fn print_dataframe(config: &Config) {
    let file = config.input_file.as_ref().unwrap();

    let mut rdr = ReaderBuilder::new()
        .delimiter(config.separator)
        .has_headers(config.has_header)
        .from_path(file)
        .unwrap();

    match config.command_type {
        CommandType::Head => print_head(config, &mut rdr),
        CommandType::Tail => print_tail(config, &mut rdr),
        _ => panic!("Invalid command type"),
    }
}

fn print_head(config: &Config, rdr: &mut csv::Reader<std::fs::File>) {
    let records: Vec<csv::StringRecord> = rdr
        .records()
        .take(config.num_rows.unwrap())
        .filter_map(Result::ok)
        .collect();

    let header = if config.has_header {
        match rdr.headers() {
            Ok(row) => Some(row),
            Err(_) => None,
        }
    } else {
        None
    };

    match config.pretty {
        true => print_clean(&records, header),
        false => print_normal(&records, header),
    }
}

fn print_tail(config: &Config, rdr: &mut csv::Reader<std::fs::File>) {
    let num_rows = rdr.records().count();
    let mut rows_to_show = config.num_rows.unwrap();
    let mut rows_to_skip = 0;

    if rows_to_show > num_rows {
        rows_to_show = num_rows;
    } else {
        rows_to_skip = num_rows - rows_to_show + if config.has_header { 1 } else { 0 };
    }

    rdr.seek(Position::new()).unwrap();

    let records = rdr
        .records()
        .skip(rows_to_skip)
        .take(rows_to_show)
        .filter_map(Result::ok)
        .collect();

    let header = if config.has_header {
        match rdr.headers() {
            Ok(row) => Some(row),
            Err(_) => None,
        }
    } else {
        None
    };

    match config.pretty {
        true => print_clean(&records, header),
        false => print_normal(&records, header),
    }
}

fn print_normal(records: &Vec<csv::StringRecord>, header: Option<&csv::StringRecord>) {
    if let Some(row) = header {
        let mut s = String::new();
        for field in row.iter() {
            s += format!("{:?},", field).as_str();
        }
        println!("{}", s.trim_end_matches(','));
    }

    for record in records.iter() {
        let mut s = String::new();
        for field in record.iter() {
            s += format!("{:?},", field).as_str();
        }
        println!("{}", s.trim_end_matches(','));
    }
}

fn print_clean(records: &Vec<csv::StringRecord>, header: Option<&csv::StringRecord>) {
    let max_lengths = get_width(records, header);

    if let Some(row) = header {
        let mut s = String::from("|");
        let mut ul = String::from("+");
        for (field, width) in row.iter().zip(max_lengths.iter()) {
            s += format!(" {:<width$} |", field, width = *width).as_str();
            ul += format!("{:-<width$}+", "", width = *width + 2).as_str();
        }
        println!("{}", s);
        println!("{}", ul);
    }

    for record in records.iter() {
        let mut s = String::from("|");
        for (field, width) in record.iter().zip(max_lengths.iter()) {
            s += format!(" {:<width$} |", field, width = *width).as_str();
        }
        println!("{}", s.trim_end_matches(','));
    }
}

fn get_width(records: &Vec<csv::StringRecord>, header: Option<&csv::StringRecord>) -> Vec<usize> {
    let mut max_lengths: Vec<usize> = Vec::new();

    // if there is a header, start with the header width
    if let Some(row) = header {
        for field in row.iter() {
            max_lengths.push(field.len());
        }
    }

    // then iterate over the records and get the max width for each column
    for record in records.iter() {
        for (i, field) in record.iter().enumerate() {
            let len = field.len();
            if i < max_lengths.len() {
                max_lengths[i] = max(max_lengths[i], len);
            } else {
                max_lengths.push(len);
            }
        }
    }

    // trucate the max length to 20
    for len in max_lengths.iter_mut() {
        if *len > 20 {
            *len = 20;
        }
    }

    max_lengths
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
    fn test_head_pretty_no_header() {
        let args = vec![
            "my_program".to_string(),
            "-f".to_string(),
            "3".to_string(),
            "--pretty".to_string(),
            "--no-header".to_string(),
            "testfile/wine.csv".to_string(),
        ];
        let config = Config::new(&args);

        print_dataframe(&config);
    }

    #[test]
    fn test_tail_pretty() {
        let args = vec![
            "my_program".to_string(),
            "-l".to_string(),
            "3".to_string(),
            "--pretty".to_string(),
            "testfile/wine.csv".to_string(),
        ];
        let config = Config::new(&args);

        print_dataframe(&config);
    }

    #[test]
    fn test_tail_pretty_no_header() {
        let args = vec![
            "my_program".to_string(),
            "-l".to_string(),
            "3".to_string(),
            "--pretty".to_string(),
            "--no-header".to_string(),
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
