use xdd::*;


use clap::Parser;

#[derive(Parser)]
struct Args {
    schema_file: String,
}
fn main() {
    let args = Args::parse();
    let src = std::fs::read_to_string(args.schema_file).unwrap();
    let v = ModelFile::from_str(src.as_str());
    let model = xdd::generate_sql(&v);
    println!("{}", model);
}


