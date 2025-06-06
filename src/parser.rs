use std::vec;

use logos::{Lexer, Logos};
use crate::{
    ast::{
        Equation,
        Expr,
        Function,
        Import,
        Literal,
        Program,
        TypeSignature,
    },
    lexer::{
        Token,
        get_precedence,
    }
};

pub fn parse(file_name: &str) -> Result<Program, String> {
    let soulite_source = std::fs::read_to_string(file_name)
        .map_err(|e| format!("Failed to read file {}: {}", file_name, e))?;
    let mut lex = Token::lexer(&soulite_source);
    let mut program = Program {
        imports: vec![],
        functions: vec![],
        variables: vec![],
    };
    loop {
        if let Some(res) = lex.next() {
            if let Ok(tok) = res {
                println!("Starting parsing: {:?}", tok);
                match tok {
                    Token::Newline | Token::Comment => continue,
                    Token::Plus => program.imports.push(parse_import(&mut lex)?),
                    Token::Dot => program.functions.push(parse_function(&mut lex)?),
                    Token::Apostrophe => program.variables.push(parse_assignment::<false>(&mut lex)?),
                    Token::Comma => program.variables.push(parse_assignment::<true>(&mut lex)?),
                    _ => return Err(format!("Unexpected token: `{}`.", lex.slice())),
                }
            } else {
                return Err(format!("Failed to parse top-level token: `{}`.", lex.slice()));
            }
        } else {
            println!("Finished parsing.");
            return Ok(program);
        }
    }
}

fn parse_import(lex: &mut Lexer<Token>) -> Result<Import, String> {
    if lex.next() != Some(Ok(Token::Identifier)) {
        return Err(format!("Expected identifier after import token `+`, but got `{}`.", lex.slice()));
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
            _ => return Err(format!("Expected identifier or new line after colon token `:`, but got `{}`.", lex.slice())),
        }
    }
    Ok(import)
}

fn parse_function(lex: &mut Lexer<Token>) -> Result<Function, String> {
    if lex.next() != Some(Ok(Token::Identifier)) {
        return Err(format!("Expected identifier after function token `.`, but got `{}`.", lex.slice()));
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
        return Err(format!("Expected `|` after function name, but got `{}`.", lex.slice()));
    }
    while lex.next() == Some(Ok(Token::Type)) {
        func.signature.arg_types.push(lex.slice().to_string());
    }
    if lex.next() != Some(Ok(Token::Arrow)) {
        return Err(format!("Expected `->` after argument types, but got `{}`.", lex.slice()));
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
            return Err(format!("Expected `'` or `,` for a function parameter, but got `{}`.", lex.slice()));
        }
        if lex.next() != Some(Ok(Token::Identifier)) {
            return Err(format!("Expected identifier for function parameter, but got `{}`.", lex.slice()));
        }
        func.equations[0].parameters_list.push(Expr::Variable(
            lex.slice().to_string()
        ));
    }
    if lex.next() != Some(Ok(Token::Assign)) {
        return Err(format!("Expected `=` after function parameters, but got `{}`.", lex.slice()));
    }
    if lex.peekable().peek() != Some(&Ok(Token::Tab)) {
        func.equations[0].body.push(parse_expression(lex)?);
    } else {
        while lex.next() == Some(Ok(Token::Tab)) {
            func.equations[0].body.push(parse_expression(lex)?);
        }
    }
    Ok(func)
}

fn parse_assignment<const IS_MUT: bool>(lex: &mut Lexer<Token>) -> Result<Expr, String> {
    if lex.next() != Some(Ok(Token::Identifier)) {
        return Err(format!("Expected identifier after assignment token, but got `{}`.", lex.slice()));
    }
    let name = lex.slice().to_string();
    if lex.next() != Some(Ok(Token::Assign)) {
        return Err(format!("Expected `=` after variable name, but got `{}`.", lex.slice()));
    }
    let value = parse_expression(lex)?;
    Ok(Expr::Assign {
        name,
        mutable: IS_MUT,
        value: Box::new(value),
    })
}

fn parse_expression(lex: &mut Lexer<Token>) -> Result<Expr, String> {
    let lhs = parse_primary(lex)?;
    parse_binary_expression(lex, lhs, 1)
}

fn parse_primary(lex: &mut Lexer<Token>) -> Result<Expr, String> {
    if let Some(Ok(tok)) = lex.next() {
        match tok {
            Token::Apostrophe => parse_assignment::<false>(lex),
            Token::Comma => parse_assignment::<true>(lex),
            Token::Identifier => parse_identifier(lex),
            Token::Float => {
                let value = lex.slice().parse::<f64>().map_err(|_| format!("Invalid float literal: `{}`.", lex.slice()))?;
                Ok(Expr::Literal(Literal::Float(value)))
            },
            Token::Integer => {
                let value = lex.slice().parse::<i64>().map_err(|_| format!("Invalid integer literal: `{}`.", lex.slice()))?;
                Ok(Expr::Literal(Literal::Integer(value)))
            },
            Token::String => {
                let value = lex.slice().trim_matches('"').to_string();
                Ok(Expr::Literal(Literal::String(value)))
            },
            Token::LeftParen => {
                let expr = parse_expression(lex)?;
                if lex.next() != Some(Ok(Token::RightParen)) {
                    return Err(format!("Expected closing parenthesis `)`, but got `{}`.", lex.slice()));
                }
                Ok(expr)
            },
            Token::Comment => parse_primary(lex),
            other => Err(format!("Unexpected token in primary expression: {:?}", other)),
        }
    } else {
        Err(format!("Expected primary expression, but got `{}`.", lex.slice()))
    }
}

fn parse_identifier(lex: &mut Lexer<Token>) -> Result<Expr, String> {
    let name = lex.slice().to_string();
    if lex.next() == Some(Ok(Token::LeftParen)) {
        let mut args = vec![];
        while let Some(Ok(tok)) = lex.next() {
            if tok == Token::RightParen {
                break;
            }
            args.push(parse_expression(lex)?);
        }
        Ok(Expr::Call { callee: name, args })
    } else {
        Ok(Expr::Variable(name))
    }
}

fn parse_binary_expression(lex: &mut Lexer<Token>, mut lhs: Expr, precedence: u8) -> Result<Expr, String> {
    while let Some(Ok(tok)) = lex.next() {
        let prec = get_precedence(&tok);
        if prec < precedence {
            return Ok(lhs);
        }
        let op = lex.slice().to_string();
        let mut rhs = parse_primary(lex)?;
        if let Some(Ok(next_tok)) = lex.next() {
            let next_prec = get_precedence(&next_tok);
            if prec < next_prec {
                rhs = parse_binary_expression(lex, rhs, next_prec + 1)?;
            }
        }
        lhs = Expr::Binary { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
    }
    Ok(lhs)
}