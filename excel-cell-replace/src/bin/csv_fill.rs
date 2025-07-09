use csv::ReaderBuilder;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use umya_spreadsheet;

/// Converts an Excel address (e.g., "B2") to (col, row) indices (1-based).
fn coordinate_from_address(addr: &str) -> Option<(u32, u32)> {
    let mut col = 0u32;
    let mut row = 0u32;
    let mut chars = addr.chars().peekable();
    // Parse column letters
    while let Some(&c) = chars.peek() {
        if c.is_ascii_alphabetic() {
            col = col * 26 + ((c.to_ascii_uppercase() as u8 - b'A' + 1) as u32);
            chars.next();
        } else {
            break;
        }
    }
    // Parse row digits
    let row_str: String = chars.collect();
    if let Ok(r) = row_str.parse::<u32>() {
        row = r;
    } else {
        return None;
    }
    if col > 0 && row > 0 {
        Some((col, row))
    } else {
        None
    }
}

/// Converts (col, row) indices (1-based) to an Excel address (e.g., (2,2) -> "B2").
fn address_from_coordinate(col: u32, row: u32) -> String {
    let mut col_num = col;
    let mut col_str = String::new();
    while col_num > 0 {
        let rem = ((col_num - 1) % 26) as u8;
        col_str.insert(0, (b'A' + rem) as char);
        col_num = (col_num - 1) / 26;
    }
    format!("{}{}", col_str, row)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        eprintln!(
            "Usage: excel-csv-fill <input.xlsx> <output.xlsx> <csvfile> <sheet> <topleftcell>"
        );
        std::process::exit(1);
    }
    let input = &args[1];
    let output = &args[2];
    let csvfile = &args[3];
    let sheet = &args[4];
    let topleft = &args[5];

    let mut book = match umya_spreadsheet::reader::xlsx::read(Path::new(input)) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Fehler beim Lesen: {}", e);
            std::process::exit(1);
        }
    };

    let ws = match book.get_sheet_by_name_mut(sheet) {
        Some(ws) => ws,
        None => {
            eprintln!("Tabelle '{}' nicht gefunden", sheet);
            std::process::exit(1);
        }
    };

    // Preprocess: filter out empty/whitespace-only lines and keep only lines with the max column count
    let lines: Vec<String> = BufReader::new(File::open(csvfile).expect("Failed to open CSV file"))
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .collect();
    let delimiter_char = if csvfile.to_lowercase().ends_with(".tsv") {
        '\t'
    } else {
        ','
    };
    let max_cols = lines
        .iter()
        .map(|line| line.split(delimiter_char).count())
        .max()
        .unwrap_or(0);
    let filtered: Vec<&String> = lines
        .iter()
        .filter(|line| line.split(delimiter_char).count() == max_cols)
        .collect();
    let filtered_data = filtered
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let delimiter = if csvfile.to_lowercase().ends_with(".tsv") {
        b'\t'
    } else {
        b','
    };
    // Use the first data row as header, only copy columns with a non-empty header
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(delimiter)
        .from_reader(filtered_data.as_bytes());

    let (start_col, start_row) = coordinate_from_address(topleft).unwrap_or((1, 1));
    let mut records = rdr.records();
    // Get header row
    let header_record = match records.next() {
        Some(Ok(rec)) => rec,
        _ => {
            eprintln!("CSV/TSV file is empty or unreadable");
            std::process::exit(1);
        }
    };
    // Indices of columns to copy (where header is not empty)
    let col_indices: Vec<usize> = header_record
        .iter()
        .enumerate()
        .filter_map(|(i, h)| {
            if !h.trim().is_empty() {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    // Write header row
    for (out_col_idx, &col_idx) in col_indices.iter().enumerate() {
        let col = start_col + out_col_idx as u32;
        let row = start_row;
        let cell_addr = address_from_coordinate(col, row);
        ws.get_cell_mut(cell_addr.as_str())
            .set_value(header_record.get(col_idx).unwrap_or(""));
    }
    // Write data rows
    for (row_idx, result) in records.enumerate() {
        let record = result.expect("Failed to read CSV record");
        if record.iter().all(|v| v.trim().is_empty()) {
            continue;
        }
        for (out_col_idx, &col_idx) in col_indices.iter().enumerate() {
            let col = start_col + out_col_idx as u32;
            let row = start_row + row_idx as u32 + 1; // +1 for header row
            let cell_addr = address_from_coordinate(col, row);
            ws.get_cell_mut(cell_addr.as_str())
                .set_value(record.get(col_idx).unwrap_or(""));
        }
    }

    if let Err(e) = umya_spreadsheet::writer::xlsx::write(&book, Path::new(output)) {
        eprintln!("Fehler beim Schreiben: {}", e);
        std::process::exit(1);
    }
    println!(
        "CSV data written to '{}' in sheet '{}' starting at {}.",
        csvfile, sheet, topleft
    );
}
