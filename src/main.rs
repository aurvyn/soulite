mod ast;
mod lexer;
mod parser;

fn main() {
    let res = parser::parse();
}
