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
        CheckToken,
        Token
    }
};

pub fn parse<const IS_DEBUG: bool>(file_name: &str) -> Result<Program, String> {
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
                if IS_DEBUG {
                    println!("Starting to parse: {:?}", lex.slice());
                }
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
            if IS_DEBUG {
                println!("Finished parsing {}.", file_name);
            }
            return Ok(program);
        }
    }
}

fn parse_import(lex: &mut Lexer<Token>) -> Result<Import, String> {
    if !lex.next().is_identifier() {
        return Err(format!("Expected identifier after import token `+`, but got `{}`.", lex.slice()));
    }
    let mut import = Import {
        filename: lex.slice().to_string(),
        items: vec![],
    };
    if lex.next().is_colon() {
        match lex.next() {
            Some(Ok(Token::Identifier)) => import.items.push(lex.slice().to_string()),
            Some(Ok(Token::Newline)) => {
                let mut tab = lex.next();
                while tab.is_tab() && lex.next().is_identifier() {
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
    if !lex.next().is_identifier() {
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
    if !lex.next().is_pipe() {
        return Err(format!("Expected `|` after function name, but got `{}`.", lex.slice()));
    }
    let mut tok = lex.next();
    while tok.is_type() {
        func.signature.arg_types.push(lex.slice().to_string());
        tok = lex.next();
    }
    if !tok.is_newline() {
        if !tok.is_arrow() {
            return Err(format!("Expected `->` after argument types, but got `{}`.", lex.slice()));
        }
        while lex.next().is_type() {
            func.signature.return_type.push(lex.slice().to_string());
        }
    }
    if func.signature.arg_types.is_empty() {
        let mut body = vec![];
        while lex.next().is_tab() {
            body.push(parse_expression(lex)?);
        }
        func.equations.push(Equation {
            parameters_list: vec![],
            body,
        });
        return Ok(func);
    }
    let mut known_param;
    for i in 0.. {
        known_param = func.signature.arg_types.len();
        func.equations.push(Equation {
            parameters_list: vec![],
            body: vec![],
        });
        for _ in 0..known_param {
            tok = lex.next();
            if !tok.is_var_marker() {
                if let Some(Ok(tok)) = tok {
                    func.equations[i].parameters_list.push(parse_literal(tok, lex.slice())?);
                    continue;
                }
                return Err(format!("Expected variable marker `'` or `,` for function parameter, but got `{}`.", lex.slice()));
            }
            known_param -= 1;
            if !lex.next().is_identifier() {
                return Err(format!("Expected identifier for function parameter, but got `{}`.", lex.slice()));
            }
            func.equations[i].parameters_list.push(Expr::Variable(
                lex.slice().to_string()
            ));
        }
        if !lex.next().is_assign() {
            return Err(format!("Expected `=` after function parameters, but got `{}`.", lex.slice()));
        }
        if !lex.next().is_newline() {
            func.equations[i].body.push(parse_expression(lex)?);
        } else {
            while lex.next().is_tab() {
                func.equations[i].body.push(parse_expression(lex)?);
            }
        }
        if known_param == 0 {
            break;
        }
    }
    Ok(func)
}

fn parse_assignment<const MUTABLE: bool>(lex: &mut Lexer<Token>) -> Result<Expr, String> {
    if !lex.next().is_identifier() {
        return Err(format!("Expected identifier after assignment token, but got `{}`.", lex.slice()));
    }
    let name = lex.slice().to_string();
    if !lex.next().is_var_assign() {
        return Err(format!("Expected assignment after variable name, but got `{}`.", lex.slice()));
    }
    let value = parse_expression(lex)?;
    Ok(Expr::Assign {
        name,
        mutable: MUTABLE,
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
            Token::Float => parse_literal(tok, lex.slice()),
            Token::Integer => parse_literal(tok, lex.slice()),
            Token::String => parse_literal(tok, lex.slice()),
            Token::LeftParen => {
                let expr = parse_expression(lex)?;
                if lex.next() != Some(Ok(Token::RightParen)) {
                    return Err(format!("Expected closing parenthesis `)`, but got `{}`.", lex.slice()));
                }
                Ok(expr)
            },
            Token::Comment => parse_primary(lex),
            other => Err(format!("Unexpected token as primary expression: {:?}", other)),
        }
    } else {
        Err(format!("Expected primary expression, but got `{}`.", lex.slice()))
    }
}

fn parse_literal(tok: Token, slice: &str) -> Result<Expr, String> {
    match tok {
        Token::Float => {
            let value = slice.parse::<f64>().unwrap();
            Ok(Expr::Literal(Literal::Float(value)))
        },
        Token::Integer => {
            let value = slice.parse::<i64>().unwrap();
            Ok(Expr::Literal(Literal::Integer(value)))
        },
        Token::String => {
            let value = slice.trim_matches('"').to_string();
            Ok(Expr::Literal(Literal::String(value)))
        },
        _ => Err(format!("Expected literal, but got `{}`.", slice)),
    }
}

fn parse_identifier(lex: &mut Lexer<Token>) -> Result<Expr, String> {
    let name = lex.slice().to_string();
    if lex.clone().next() == Some(Ok(Token::LeftParen)) {
        lex.next();
        let mut args = vec![];
        while lex.clone().next() != Some(Ok(Token::RightParen)) {
            args.push(parse_expression(lex)?);
        }
        Ok(Expr::Call { callee: name, args })
    } else {
        Ok(Expr::Variable(name))
    }
}

fn parse_binary_expression(lex: &mut Lexer<Token>, mut lhs: Expr, precedence: u8) -> Result<Expr, String> {
    while let Some(Ok(tok)) = lex.clone().next() {
        let prec = tok.get_precedence();
        if prec < precedence {
            return Ok(lhs);
        }
        lex.next();
        let op = lex.slice().to_string();
        let mut rhs = parse_primary(lex)?;
        if let Some(Ok(next_tok)) = lex.next() {
            let next_prec = next_tok.get_precedence();
            if prec < next_prec {
                rhs = parse_binary_expression(lex, rhs, next_prec + 1)?;
            }
        }
        lhs = Expr::Binary { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
    }
    Ok(lhs)
}
