use clap::{Parser, ValueEnum};
use pjl_pandoc::{Block, Inline, Pandoc};
use serde_json;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(
    name = "itfmd",
    about = "Convert SAPscript ITF from or to Markdown, or to Pandoc JSON"
)]
struct Cli {
    #[arg(value_enum)]
    mode: Mode,
    /// Maximum line length for markdown output (default: 80)
    #[arg(long, default_value_t = 80)]
    max_line_length: usize,
}

#[derive(ValueEnum, Clone)]
enum Mode {
    Itf2md,
    Md2itf,
    Itf2json,
}

fn parse_inlines(content: &str) -> Vec<Inline> {
    let mut inlines = Vec::new();
    let mut chars = content.chars().peekable();
    let mut buffer = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            // Flush buffer
            if !buffer.is_empty() {
                inlines.push(Inline::Str(buffer.to_string()));
                buffer.clear();
            }
            // Add space
            inlines.push(Inline::Space);
            chars.next();
        } else if c == '<' {
            // Flush buffer
            if !buffer.is_empty() {
                inlines.push(Inline::Str(buffer.to_string()));
                buffer.clear();
            }
            // Parse markup
            chars.next(); // consume '<'
            let mut tag = String::new();
            while let Some(&tc) = chars.peek() {
                if tc == '>' {
                    chars.next();
                    break;
                }
                tag.push(tc);
                chars.next();
            }
            if tag == "(" {
                // Literal mode: <(>...<)>
                let mut literal = String::new();
                while let Some(&lc) = chars.peek() {
                    if lc == '<' {
                        let mut close_tag = String::new();
                        let mut temp = chars.clone();
                        temp.next(); // consume '<'
                        while let Some(&ctc) = temp.peek() {
                            if ctc == '>' {
                                temp.next();
                                break;
                            }
                            close_tag.push(ctc);
                            temp.next();
                        }
                        if close_tag == ")" {
                            for _ in 0..(close_tag.len() + 2) {
                                chars.next();
                            }
                            break;
                        }
                    }
                    literal.push(lc);
                    chars.next();
                }
                if !literal.is_empty() {
                    inlines.push(Inline::Str(literal));
                }
            } else if tag.starts_with('/') {
                // End tag, ignore
            } else {
                // Highlight or other markup: <ZH>...</>
                // let mut inner = String::new();
                // while let Some(&ic) = chars.peek() {
                //     if ic == '<' {
                //         let mut close_tag = String::new();
                //         let mut temp = chars.clone();
                //         temp.next();
                //         while let Some(&ctc) = temp.peek() {
                //             if ctc == '>' {
                //                 temp.next();
                //                 break;
                //             }
                //             close_tag.push(ctc);
                //             temp.next();
                //         }
                //         if close_tag == format!("/{}", tag) {
                //             for _ in 0..(close_tag.len() + 2) {
                //                 chars.next();
                //             }
                //             break;
                //         }
                //     }
                //     inner.push(ic);
                //     chars.next();
                // }
                // // Recursively parse inner markup
                // let nested = parse_inlines(&inner);
                // match tag.as_str() {
                //     "ZH" => inlines.push(Inline::Strong(nested)),
                //     "ZK" => inlines.push(Inline::Emph(nested)),
                //     _ => inlines.extend(nested),
                // }
            }
        } else {
            buffer.push(c);
            chars.next();
        }
    }
    // Flush remaining buffer
    if !buffer.is_empty() {
        inlines.push(Inline::Str(buffer.to_string()));
    }
    inlines
}

fn itf_to_pandoc(input: &str) -> Pandoc {
    let mut blocks = Vec::new();
    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        if line.len() < 2 {
            blocks.push(Block::Para(vec![]));
            continue;
        }
        let ctrl = line_command(line);
        let mut content = line[2..].trim_end().to_string();
        // Collect continuation lines (ctrl == "  ")
        while let Some(&next_line) = lines.peek() {
            let command = line_command(next_line);
            match command[..] {
                [' ', ' '] => {
                    content.push_str(" ");
                    content.push_str(line_content(next_line));
                    lines.next();
                }
                ['=', ' '] => {
                    content.push_str(line_content(next_line));
                    lines.next();
                }
                _ => {
                    break;
                }
            }
        }

        let inlines = parse_inlines(&content);
        match ctrl[..] {
            ['U', '1'] => {
                if !inlines.is_empty() {
                    blocks.push(Block::Header((2, (String::new(), vec![], vec![]), inlines)));
                } else {
                    blocks.push(Block::Para(vec![]));
                }
            }
            ['*', ' '] => {
                if !inlines.is_empty() {
                    blocks.push(Block::Para(inlines));
                } else {
                    blocks.push(Block::Para(vec![]));
                }
            }
            _ => {
                blocks.push(Block::Para(inlines));
            }
        }
    }
    Pandoc::new(blocks)
}

fn line_command(next_line: &str) -> Vec<char> {
    let mut chars = next_line.chars().take(2).collect::<Vec<_>>();
    while chars.len() < 2 {
        chars.push(' ');
    }
    chars
}

fn line_content(line: &str) -> &str {
    if line.len() >= 2 {
        &line[2..].trim()
    } else {
        ""
    }
}

fn render_inlines_to_string(inlines: &[Inline]) -> String {
    let mut out = String::new();
    for inline in inlines {
        match inline {
            Inline::Str(s) => out.push_str(s),
            Inline::Space => out.push(' '),
            Inline::Emph(e) => out.push_str(&format!("*{}*", render_inlines_to_string(e))),
            Inline::Strong(e) => out.push_str(&format!("**{}**", render_inlines_to_string(e))),
            Inline::Strikeout(e) => out.push_str(&format!("~~{}~~", render_inlines_to_string(e))),
            Inline::Superscript(e) => out.push_str(&format!("^{}", render_inlines_to_string(e))),
            Inline::Subscript(e) => out.push_str(&format!("~{}", render_inlines_to_string(e))),
            Inline::SmallCaps(e) => out.push_str(&render_inlines_to_string(e)),
            Inline::Quoted((_, e)) => out.push_str(&format!("\"{}\"", render_inlines_to_string(e))),
            Inline::Cite((_, e)) => out.push_str(&render_inlines_to_string(e)),
            Inline::Code((_, s)) => out.push_str(&format!("`{}`", s)),
            Inline::SoftBreak | Inline::LineBreak => out.push(' '),
            Inline::Math((_, s)) => out.push_str(s),
            Inline::RawInline((_, s)) => out.push_str(s),
            Inline::Link((_, e, _)) => out.push_str(&render_inlines_to_string(e)),
            Inline::Image((_, e, _)) => out.push_str(&render_inlines_to_string(e)),
            Inline::Note(_) => out.push_str("[Note]"),
            Inline::Span((_, e)) => out.push_str(&render_inlines_to_string(e)),
        }
    }
    out
}

fn pandoc_to_itf(doc: &Pandoc) -> String {
    let mut out = String::new();
    for block in &doc.blocks {
        match block {
            Block::Header((2, _, inlines)) => {
                let text = render_inlines_to_string(inlines);
                for chunk in text.as_bytes().chunks(72) {
                    out.push_str(&format!("U1{}\n", String::from_utf8_lossy(chunk)));
                }
            }
            Block::Para(inlines) => {
                let text = render_inlines_to_string(inlines);
                if text.is_empty() {
                    out.push_str("* \n");
                } else {
                    for chunk in text.as_bytes().chunks(72) {
                        out.push_str(&format!("* {}\n", String::from_utf8_lossy(chunk)));
                    }
                }
            }
            Block::CodeBlock((_, code)) => {
                for chunk in code.as_bytes().chunks(72) {
                    out.push_str(&format!("  {}\n", String::from_utf8_lossy(chunk)));
                }
            }
            _ => {}
        }
    }
    out
}

fn render_inlines_to_markdown(inlines: &[Inline]) -> String {
    let mut out = String::new();
    for inline in inlines {
        match inline {
            Inline::Str(s) => out.push_str(s),
            Inline::Space => out.push(' '),
            Inline::Emph(e) => out.push_str(&format!("*{}*", render_inlines_to_markdown(e))),
            Inline::Strong(e) => out.push_str(&format!("**{}**", render_inlines_to_markdown(e))),
            Inline::Strikeout(e) => out.push_str(&format!("~~{}~~", render_inlines_to_markdown(e))),
            Inline::Superscript(e) => out.push_str(&format!("^{}", render_inlines_to_markdown(e))),
            Inline::Subscript(e) => out.push_str(&format!("~{}", render_inlines_to_markdown(e))),
            Inline::SmallCaps(e) => out.push_str(&render_inlines_to_markdown(e)),
            Inline::Quoted((_, e)) => {
                out.push_str(&format!("\"{}\"", render_inlines_to_markdown(e)))
            }
            Inline::Cite((_, e)) => out.push_str(&render_inlines_to_markdown(e)),
            Inline::Code((_, s)) => out.push_str(&format!("`{}`", s)),
            Inline::SoftBreak | Inline::LineBreak => out.push('\n'),
            Inline::Math((_, s)) => out.push_str(s),
            Inline::RawInline((_, s)) => out.push_str(s),
            Inline::Link((_, e, _)) => out.push_str(&render_inlines_to_markdown(e)),
            Inline::Image((_, e, _)) => out.push_str(&render_inlines_to_markdown(e)),
            Inline::Note(_) => out.push_str("[Note]"),
            Inline::Span((_, e)) => out.push_str(&render_inlines_to_markdown(e)),
        }
    }
    out
}

fn split_text_by_length(text: &str, max_len: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.chars().count() + 1 + word.chars().count() <= max_len {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn pandoc_to_markdown(doc: &Pandoc, max_len: usize) -> String {
    let mut out = String::new();
    for block in &doc.blocks {
        match block {
            Block::Header((2, _, inlines)) => {
                let text = render_inlines_to_markdown(inlines);
                for line in split_text_by_length(&text, max_len) {
                    out.push_str(&format!("## {}\n", line));
                }
            }
            Block::Para(inlines) => {
                let text = render_inlines_to_markdown(inlines);
                if !text.is_empty() {
                    for line in split_text_by_length(&text, max_len) {
                        out.push_str(&format!("{}\n", line));
                    }
                }
                out.push_str("\n");
            }
            Block::CodeBlock((_, code)) => {
                for line in split_text_by_length(code, max_len) {
                    out.push_str(&format!("    {}\n", line));
                }
            }
            _ => {}
        }
    }
    out
}

fn markdown_to_pandoc(input: &str) -> Pandoc {
    let mut blocks = Vec::new();
    for line in input.lines() {
        let trimmed = line.trim_end();
        if trimmed.starts_with("## ") {
            blocks.push(Block::Header((
                2,
                (String::new(), vec![], vec![]),
                vec![Inline::Str(trimmed[3..].to_string())],
            )));
        } else if trimmed.is_empty() {
            blocks.push(Block::Para(vec![]));
        } else if trimmed.starts_with("    ") {
            blocks.push(Block::CodeBlock((
                (String::new(), vec![], vec![]),
                trimmed[4..].to_string(),
            )));
        } else {
            blocks.push(Block::Para(vec![Inline::Str(trimmed.to_string())]));
        }
    }
    Pandoc::new(blocks)
}

fn main() {
    let cli = Cli::parse();
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");
    match cli.mode {
        Mode::Itf2md => {
            let doc = itf_to_pandoc(&input);
            let out = pandoc_to_markdown(&doc, cli.max_line_length);
            io::stdout()
                .write_all(out.as_bytes())
                .expect("Failed to write markdown");
        }
        Mode::Md2itf => {
            let doc = markdown_to_pandoc(&input);
            let out = pandoc_to_itf(&doc);
            io::stdout()
                .write_all(out.as_bytes())
                .expect("Failed to write ITF");
        }
        Mode::Itf2json => {
            let doc = itf_to_pandoc(&input);
            serde_json::to_writer(io::stdout(), &doc).expect("Failed to write pandoc JSON");
        }
    }
}
