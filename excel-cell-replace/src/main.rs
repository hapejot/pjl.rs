use std::env;
use std::path::Path;
use umya_spreadsheet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        eprintln!("Usage: excel-cell-replace <input.xlsx> <output.xlsx> <sheet> <cell> <value>");
        std::process::exit(1);
    }
    let input = &args[1];
    let output = &args[2];
    let sheet = &args[3];
    let cell = &args[4];
    let value = &args[5];

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

    ws.get_cell_mut(cell.as_str()).set_value(value);

    if let Err(e) = umya_spreadsheet::writer::xlsx::write(&book, Path::new(output)) {
        eprintln!("Fehler beim Schreiben: {}", e);
        std::process::exit(1);
    }
    println!("Zelle {} in '{}' wurde auf '{}' gesetzt.", cell, sheet, value);
}
