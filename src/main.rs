use std::{fs::File, io::Write};

mod ast;
mod lexer;
mod parser;

fn main() -> Result<(), String> {
    let tree = parser::parse("test/expr.soul")?;
    let mut file = File::create("test/expr.rs")
        .map_err(|e| format!("Failed to create output file: {}", e))?;
    file.write_all(tree.to_rust().as_bytes())
        .map_err(|e| format!("Failed to write to output file: {}", e))?;
    Ok(())
}
