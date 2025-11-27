use std::{fs::File, io::Write, process::Command};

use clap::Parser;

use crate::ast::ToRust;

mod ast;
mod lexer;
mod parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Path to the Soulite file for compiling
    ///
    /// Example: `soulite main.sl`
    soulite_file: String,

    /// Set custom path for the outputted Rust file
    ///
    /// Example: `soulite -t build/main.rs src/main.sl`
    #[arg(short, long, value_name = "OUTPUT.rs")]
    transpile: Option<String>,

    /// Enable direct compilation and set output file path (requires `rustc`)
    ///
    /// Example: `soulite -c build/main src/main.sl`
    #[arg(short, long, value_name = "EXE")]
    compile: Option<String>,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let soulite_tree = parser::parse::<false>(&cli.soulite_file)?;
    let rust_tree = syn::parse_file(&soulite_tree.to_rust()).unwrap();
    let rust_code = prettyplease::unparse(&rust_tree);
    let rust_file = cli
        .transpile
        .unwrap_or_else(|| cli.soulite_file.replace(".sl", ".rs"));
    let mut file = File::create(&rust_file).map_err(|e| format!("{}", e))?;
    file.write_all(rust_code.as_bytes())
        .map_err(|e| format!("Failed to write to output file: {}", e))?;
    if let Some(file_name) = cli.compile
        && Command::new("rustc")
            .arg(rust_file)
            .arg("-o")
            .arg(&file_name)
            .output()
            .unwrap()
            .status
            .success()
    {
        println!("Compiled successfully to `{}`.", file_name)
    }
    Ok(())
}
