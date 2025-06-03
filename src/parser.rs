use std::vec;

use logos::{Lexer, Logos};
use crate::{
    ast::{
        Equation,
        Expr,
        Function,
        Import,
        Program,
        TypeSignature,
    },
    lexer::Token
};

pub fn parse() -> Result<Program, String> {
    let mut lex = Token::lexer("Soulite Lexer");
    let mut program = Program {
        imports: vec![],
        functions: vec![],
        variables: vec![],
    };
    loop {
        if let Some(res) = lex.next() {
            if let Ok(tok) = res {
                match tok {
                    Token::Newline => continue,
                    Token::Plus => program.imports.push(parse_import(&mut lex)?),
                    Token::Dot => program.functions.push(parse_function(&mut lex)?),
                    Token::Apostrophe => program.variables.push(parse_assignment(&mut lex)?),
                    token => return Err(format!("Unexpected token: {:?}", token)),
                }
            } else {
                return Err("Failed to parse top-level token".to_string());
            }
        } else {
            println!("Ending parsing.");
            return Ok(program);
        }
    }
}

fn parse_import<'src>(lex: &mut Lexer<Token>) -> Result<Import, &'src str> {
    if lex.next() != Some(Ok(Token::Identifier)) {
        return Err("Expected identifier after import token `+`.");
    }
    let mut import = Import {
        filename: lex.slice().to_string(),
        items: vec![],
    };
    if lex.next() == Some(Ok(Token::Colon)) {
        match lex.next() {
            Some(Ok(Token::Identifier)) => import.items.push(lex.slice().to_string()),
            Some(Ok(Token::Newline)) => {
                let mut tab = lex.next();
                while tab == Some(Ok(Token::Tab))
                && lex.next() == Some(Ok(Token::Identifier)) {
                    import.items.push(lex.slice().to_string());
                    tab = lex.next();
                }
            }
            _ => return Err("Expected identifier or new line after colon token `:`."),
        }
    }
    Ok(import)
}

fn parse_function<'src>(lex: &mut Lexer<Token>) -> Result<Function, &'src str> {
    if lex.next() != Some(Ok(Token::Identifier)) {
        return Err("Expected identifier after function token `.`.");
    }
    let mut func = Function {
        signature: TypeSignature {
            name: lex.slice().to_string(),
            arg_types: vec![],
            return_type: vec![],
        },
        equations: vec![],
    };
    if lex.next() != Some(Ok(Token::Pipe)) {
        return Err("Expected `|` after function name.");
    }
    while lex.next() == Some(Ok(Token::Type)) {
        func.signature.arg_types.push(lex.slice().to_string());
    }
    if lex.next() != Some(Ok(Token::Arrow)) {
        return Err("Expected `->` after argument types.");
    }
    while lex.next() == Some(Ok(Token::Type)) {
        func.signature.return_type.push(lex.slice().to_string());
    }
    func.equations.push(Equation {
        parameters_list: vec![],
        body: vec![],
    });
    for _ in 0..func.signature.arg_types.len() {
        let var_indicator = lex.next();
        if var_indicator != Some(Ok(Token::Apostrophe))
        || var_indicator != Some(Ok(Token::Identifier)) {
            return Err("Expected `'` or `,` for a function parameter.");
        }
        if lex.next() != Some(Ok(Token::Identifier)) {
            return Err("Expected identifier for function parameter.");
        }
        func.equations[0].parameters_list.push(Expr::Variable(
            lex.slice().to_string()
        ));
    }
    Ok(func)
}

fn parse_assignment<'src>(lex: &mut Lexer<Token>) -> Result<Expr, &'src str> {
    Err("Not implemented yet.")
}