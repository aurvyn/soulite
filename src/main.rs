use std::{fs::File, io::Write};

use crate::ast::ToRust;

mod ast;
mod lexer;
mod parser;

fn main() -> Result<(), String> {
    let soulite_tree = parser::parse::<true>("test/func.sl")?;
    let rust_tree = syn::parse_file(&soulite_tree.to_rust()).unwrap();
    let rust_code = prettyplease::unparse(&rust_tree);
    let mut file =
        File::create("test/func.rs").map_err(|e| format!("Failed to create output file: {}", e))?;
    file.write_all(rust_code.as_bytes())
        .map_err(|e| format!("Failed to write to output file: {}", e))?;
    Ok(())
}
