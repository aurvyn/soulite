use std::{fs::File, io::Write};

use crate::ast::ToRust;

mod ast;
mod lexer;
mod parser;

fn main() -> Result<(), String> {
    let tree = parser::parse::<true>("test/func.soul")?;
    let mut file = File::create("test/func.rs")
        .map_err(|e| format!("Failed to create output file: {}", e))?;
    file.write_all(tree.to_rust().as_bytes())
        .map_err(|e| format!("Failed to write to output file: {}", e))?;
    Ok(())
}
