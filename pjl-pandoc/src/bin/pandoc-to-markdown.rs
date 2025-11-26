use pjl_pandoc::{Block, Inline};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use pjl_pandoc::Pandoc;
    let doc: Pandoc = serde_json::from_reader(std::io::stdin())?;

    write_document(doc, &mut std::io::stdout())?;

    Ok(())
}

fn write_document<O>(
    doc: pjl_pandoc::Pandoc,
    stdout: &mut O,
) -> Result<(), Box<dyn std::error::Error>>
where
    O: std::io::Write,
{
    for block in doc.blocks {
        match block {
            Block::Para(inlines) => {
                for inline in inlines {
                    match inline {
                        Inline::Str(s) => {
                            write!(stdout, "{}", s)?;
                        }
                        Inline::Space => {
                            write!(stdout, " ")?;
                        }
                        Inline::SoftBreak => {
                            write!(stdout, " ")?;
                        }
                        x => {
                            todo!("inline: {:?}", x)
                        }
                    }
                }
                writeln!(stdout)?;
            }
            x => {
                todo!("block: {:?}", x)
            }
        }
    }
    Ok(())
}
