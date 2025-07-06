use std::{fs::File, io::Write, process::Command};

use crate::ast::ToRust;

mod ast;
mod lexer;
mod parser;

fn main() -> Result<(), String> {
    let mut args = std::env::args();
    let cmd_name = args.next().unwrap();
    let soulite_file = args.next().expect("Please provide a Soulite file name");
    let soulite_tree = parser::parse::<true>(&soulite_file)?;
    let rust_tree = syn::parse_file(&soulite_tree.to_rust()).unwrap();
    let rust_code = prettyplease::unparse(&rust_tree);
    let mut rust_file = None;
    let mut out_file = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => println!("Usage: {cmd_name} <input_file.sl>"),
            "--transpile" | "-t" => {
                if let Some(output_file) = args.next() {
                    rust_file = Some(output_file);
                } else {
                    return Err("No rust file name specified".to_string());
                }
            }
            "--compile" | "-c" => {
                if let Some(output_file) = args.next() {
                    out_file = Some(output_file);
                } else {
                    return Err("No output file specified".to_string());
                }
            }
            _ => return Err(format!("Unknown argument: {}", arg)),
        }
    }
    if rust_file.is_none() {
        rust_file = Some(soulite_file.replace(".sl", ".rs"));
    }
    let mut file = File::create(rust_file.as_ref().unwrap()).map_err(|e| format!("{}", e))?;
    file.write_all(rust_code.as_bytes())
        .map_err(|e| format!("Failed to write to output file: {}", e))?;
    if let Some(file_name) = out_file {
        Command::new("rustc")
            .arg(rust_file.as_ref().unwrap())
            .arg("-o")
            .arg(&file_name)
            .output()
            .map_err(|e| format!("Failed to compile: {}", e))?;
    }
    Ok(())
}
