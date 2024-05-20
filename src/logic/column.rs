use crate::{enums::commands::CommandType, structs::config::Config};
use csv::ReaderBuilder;
use std::{cmp::max, cmp::min};

const MAX_WIDTH: usize = 20;

fn find_index_of_column(header: &csv::StringRecord, column_name: &str) -> Option<usize> {
    header.iter().position(|x| x == column_name)
}

fn ellipsis(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len - 3])
    } else {
        s.to_string()
    }
}

fn get_width(
    records: &Vec<csv::StringRecord>,
    header: Option<&csv::StringRecord>,
    column_index: usize,
) -> usize {
    let mut max_lengths = 0;

    // if there is a header, start with the header width
    if let Some(row) = header {
        max_lengths = row[column_index].len();
    }

    // then iterate over the records and get the max width for each column
    for record in records.iter() {
        let len = record[column_index].len();
        max_lengths = max(max_lengths, len);
    }

    // trucate the max length to MAX_WIDTH
    min(max_lengths, MAX_WIDTH)
}

fn print_normal(
    records: &Vec<csv::StringRecord>,
    header: Option<&csv::StringRecord>,
    column_index: usize,
) {
    if let Some(row) = header {
        println!("{:?}", row.get(column_index).unwrap());
    }

    for record in records.iter() {
        println!("{:?}", record.get(column_index).unwrap());
    }
}

fn print_clean(
    records: &Vec<csv::StringRecord>,
    header: Option<&csv::StringRecord>,
    column_index: usize,
) {
    let max_lengths = get_width(records, header, column_index);

    if let Some(row) = header {
        println!(
            "| {:<width$} |",
            ellipsis(&row[column_index], MAX_WIDTH),
            width = max_lengths
        );
        println!("+{:-<width$}+", "", width = max_lengths + 2);
    }

    for record in records.iter() {
        println!(
            "| {:<width$} |",
            ellipsis(&record[column_index], MAX_WIDTH),
            width = max_lengths
        );
    }
}

pub fn print_column(config: &Config) {
    let file = config.input_file.as_ref().unwrap();

    let mut rdr = ReaderBuilder::new()
        .delimiter(config.separator)
        .has_headers(config.has_header)
        .from_path(file)
        .unwrap();

    let records = rdr
        .records()
        .collect::<csv::Result<Vec<csv::StringRecord>>>()
        .unwrap();

    let header = match rdr.headers() {
        Ok(row) => Some(row),
        Err(_) => None,
    };

    let column_index = match config.command_type {
        CommandType::ColumnName => {
            find_index_of_column(header.unwrap(), config.column_name.as_ref().unwrap())
        }
        CommandType::ColumnIndex => Some(config.column_index.unwrap() - 1),
        _ => None,
    };

    match config.pretty {
        true => print_clean(&records, header, column_index.unwrap()),
        false => print_normal(&records, header, column_index.unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_by_index() {
        let args = vec![
            "my_program".to_string(),
            "-i".to_string(),
            "3".to_string(),
            "testfile/wine.csv".to_string(),
        ];
        let config = Config::new(&args);

        print_column(&config);
    }

    #[test]
    fn test_column_by_name() {
        let args = vec![
            "my_program".to_string(),
            "-n".to_string(),
            "Proanth".to_string(),
            "testfile/wine.csv".to_string(),
        ];
        let config = Config::new(&args);

        print_column(&config);
    }

    #[test]
    fn test_missing_column() {
        let args = vec![
            "my_program".to_string(),
            "-n".to_string(),
            "NOTFOUND".to_string(),
            "testfile/wine.csv".to_string(),
        ];
        let config = Config::new(&args);

        print_column(&config);
    }

    #[test]
    fn test_index_to_high() {
        let args = vec![
            "my_program".to_string(),
            "-i".to_string(),
            "999".to_string(),
            "testfile/wine.csv".to_string(),
        ];
        let config = Config::new(&args);

        print_column(&config);
    }

    #[test]
    fn test_column_pretty() {
        let args = vec![
            "my_program".to_string(),
            "-n".to_string(),
            "Proanth".to_string(),
            "--pretty".to_string(),
            "testfile/wine.csv".to_string(),
        ];
        let config = Config::new(&args);

        print_column(&config);
    }
}
