use std::env;
use std::fs;

fn sapscript_to_markdown(input: &str) -> String {
    let mut out = String::new();
    for line in input.lines() {
        if line.len() < 2 {
            out.push_str("\n");
            continue;
        }
        let ctrl = &line[..2];
        let content = line[2..].trim_end();
        match ctrl {
            "U1" => {
                if !content.is_empty() {
                    out.push_str(&format!("## {}\n", content));
                } else {
                    out.push_str("\n");
                }
            }
            "* " => {
                if !content.is_empty() {
                    out.push_str(&format!("{}\n", content));
                } else {
                    out.push_str("\n");
                }
            }
            "  " => {
                // Indented/continued line
                out.push_str(&format!("    {}\n", content));
            }
            _ => {
                // Unknown control code, treat as plain text
                out.push_str(&format!("{}\n", content));
            }
        }
    }
    out
}

fn markdown_to_sapscript(input: &str) -> String {
    let mut out = String::new();
    for line in input.lines() {
        let trimmed = line.trim_end();
        if trimmed.starts_with("## ") {
            out.push_str(&format!("U1{}\n", &trimmed[3..]));
        } else if trimmed.is_empty() {
            out.push_str("* \n");
        } else if trimmed.starts_with("    ") {
            out.push_str(&format!("  {}\n", &trimmed[4..]));
        } else {
            out.push_str(&format!("* {}\n", trimmed));
        }
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: sapmd <to-md|to-sap> <inputfile> <outputfile>");
        std::process::exit(1);
    }
    let mode = &args[1];
    let inputfile = &args[2];
    let outputfile = &args[3];
    let input = fs::read_to_string(inputfile).expect("Failed to read input file");
    let output = match mode.as_str() {
        "to-md" => sapscript_to_markdown(&input),
        "to-sap" => markdown_to_sapscript(&input),
        _ => {
            eprintln!("Unknown mode: {}", mode);
            std::process::exit(1);
        }
    };
    fs::write(outputfile, output).expect("Failed to write output file");
    println!("Done: {} -> {}", inputfile, outputfile);
}
