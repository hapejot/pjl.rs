# pjl-pandoc

A Rust library for reading and writing Pandoc JSON documents, providing basic Pandoc AST types and serde (de)serialization support.

## Features
- Read and write Pandoc JSON using serde/serde_json
- Basic Pandoc AST types (Block, Inline, etc.)
- Easily extendable for more Pandoc features

## Usage
Add to your `Cargo.toml`:

```toml
pjl-pandoc = { path = "../pjl-pandoc" }
```

Example:
```rust
use pjl_pandoc::Pandoc;
let doc: Pandoc = serde_json::from_reader(std::fs::File::open("doc.json")?)?;
serde_json::to_writer(std::fs::File::create("out.json")?, &doc)?;
```

## License
MIT
