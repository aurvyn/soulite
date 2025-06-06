mod ast;
mod lexer;
mod parser;

fn main() -> Result<(), String> {
    let tree = parser::parse("test/expr.soul")?;
    println!("{}", tree.to_rust());
    Ok(())
}
