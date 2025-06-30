use std::{fs::File, io::Write};

use crate::ast::ToRust;

mod ast;
mod lexer;
mod parser;

fn main() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 1 || args[1] == "-h" || args[1] == "--help" {
        return Err(format!("Usage: {} <file.sl>", args[0]));
    }
    let soulite_tree = parser::parse::<true>(&args[1])?;
    let rust_tree = syn::parse_file(&soulite_tree.to_rust()).unwrap();
    let rust_code = prettyplease::unparse(&rust_tree);
    let output_file = args[1].replace(".sl", ".rs");
    let mut file = File::create(output_file).map_err(|e| format!("{}", e))?;
    file.write_all(rust_code.as_bytes())
        .map_err(|e| format!("Failed to write to output file: {}", e))?;
    Ok(())
}
