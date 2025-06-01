use logos::{Lexer, Logos};
use crate::lexer::Token;

pub fn parse() {
    let mut lex = Token::lexer("Soulite Lexer");
    loop {
        if let Some(res) = lex.next() {
            if let Ok(tok) = res {
                match tok {
                    Token::Dot => parse_definition(&mut lex),
                    _ => parse_top_level(&mut lex),
                }
            }
        } else {
            println!("Ending parsing.");
            break;
        }
    }
}

fn parse_definition(lex: &mut Lexer<Token>) {
    if let Some(token) = lex.next() {
        println!("Definition token: {:?}", token);
    }
}

fn parse_top_level(lex: &mut Lexer<Token>) {
    if let Some(token) = lex.next() {
        println!("Top-level token: {:?}", token);
    }
}