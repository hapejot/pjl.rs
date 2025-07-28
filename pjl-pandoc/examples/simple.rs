fn main() -> Result<(), Box<dyn std::error::Error>> {
    use pjl_pandoc::Pandoc;
    let doc: Pandoc = serde_json::from_reader(std::io::stdin())?;
    // println!("{:#?}", doc);
    serde_json::to_writer(std::io::stdout(), &doc)?;
    Ok(())
}
