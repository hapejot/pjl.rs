use pjl_tab::Table;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // JSON von stdin lesen
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // JSON als Table deserialisieren
    let table: Table = serde_json::from_str(&input)?;

    // Als Markdown-Tabelle ausgeben
    let mut output = String::new();
    table.dump(&mut output);
    print!("{}", output);

    Ok(())
}
