use std::vec;

use crate::{
    ast::{
        Equation, Expr, Function, Import, Literal, Pattern, Program, Struct, Type, TypeSignature,
    },
    lexer::{CheckToken, Lookahead, Token},
};
use logos::{Lexer, Logos};

pub fn parse<const IS_DEBUG: bool>(file_name: &str) -> Result<Program, String> {
    let soulite_source = std::fs::read_to_string(file_name)
        .map_err(|e| format!("Failed to read file {}: {}", file_name, e))?;
    let mut lex = Token::lexer(&soulite_source);
    let mut program = Program {
        imports: vec![],
        structs: vec![],
        functions: vec![],
        variables: vec![],
    };
    loop {
        let Some(res) = lex.next() else {
            if IS_DEBUG {
                println!("Finished parsing {}.", file_name);
            }
            return Ok(program);
        };
        let Ok(tok) = res else {
            return err(&lex, "top-level token");
        };
        if IS_DEBUG {
            println!("Starting to parse: {:?}", lex.slice());
        }
        match tok {
            Token::Newline | Token::Comment => continue,
            Token::Plus => program.imports.push(parse_import(&mut lex)?),
            Token::Identifier => {
                let name = lex.slice().to_string();
                let Some(Ok(tok)) = lex.next() else {
                    return err(&lex, "token after identifier");
                };
                match tok {
                    Token::Pipe => program.functions.push(parse_function(&mut lex, name)?),
                    Token::Apostrophe => program
                        .variables
                        .push(parse_assignment::<false>(&mut lex, name)?),
                    Token::Comma => program
                        .variables
                        .push(parse_assignment::<true>(&mut lex, name)?),
                    _ => return err(&lex, "variable or function marker"),
                }
            }
            Token::Type => program.structs.push(parse_struct(&mut lex)?),
            _ => return err(&lex, "import or declaration"),
        }
    }
}

fn parse_import(lex: &mut Lexer<Token>) -> Result<Import, String> {
    if !lex.next().is_identifier() {
        return err(lex, "identifier after import token `+`");
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
            _ => return err(lex, "identifier or new line after colon token `:`"),
        }
    }
    Ok(import)
}

fn parse_struct(lex: &mut Lexer<Token>) -> Result<Struct, String> {
    let mut result = Struct {
        name: lex.slice().to_string(),
        fields: vec![],
    };
    let mut tok = lex.next();
    let generic_types = if tok == Some(Ok(Token::LessThan)) {
        let result = parse_generic_types(lex)?;
        tok = lex.next();
        result
    } else {
        vec![]
    };
    match tok {
        Some(Ok(Token::FatArrow)) => {
            // trait impl, not yet implemented
        }
        Some(Ok(Token::Assign)) => {}
        _ => return err(&lex, "colon, arrow, or generic type after struct name"),
    }
    while lex.next().is_newline() && lex.next().is_tab() {
        if !lex.next().is_identifier() {
            return err(&lex, "field name after tab");
        }
        let field_name = lex.slice().to_string();
        if !lex.next().is_type() {
            return err(&lex, "field type");
        }
        result
            .fields
            .push((field_name, parse_type(lex, &generic_types)?));
    }
    Ok(result)
}

fn parse_function(lex: &mut Lexer<Token>, name: String) -> Result<Function, String> {
    let mut func = Function {
        signature: TypeSignature {
            name,
            arg_types: vec![],
            return_types: vec![],
        },
        equations: vec![],
    };
    let mut tok = lex.next();
    while tok.is_type() {
        func.signature.arg_types.push(parse_type(lex, &vec![])?);
        tok = lex.next();
    }
    if !tok.is_newline() {
        if !tok.is_arrow() {
            return err(lex, "`->` after argument types");
        }
        while lex.next().is_type() {
            func.signature.return_types.push(parse_type(lex, &vec![])?);
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
            let pattern = parse_parameter(lex)?;
            if matches!(pattern, Pattern::Variable(_) | Pattern::Wildcard) {
                known_param -= 1;
            }
            func.equations[i].parameters_list.push(pattern);
        }
        if !lex.next().is_colon() {
            return err(lex, "`:` after function parameters");
        }
        if !lex.peek().is_newline() {
            func.equations[i].body.push(parse_expression(lex)?);
            if !lex.next().is_newline() {
                return err(lex, "newline after function body expression");
            }
        } else {
            while lex.peek().is_newline() {
                lex.next();
                if !lex.peek().is_tab() {
                    break;
                }
                lex.next();
                func.equations[i].body.push(parse_expression(lex)?);
            }
        }
        if known_param == 0 {
            break;
        }
    }
    Ok(func)
}

fn parse_generic_types(lex: &mut Lexer<Token>) -> Result<Vec<String>, String> {
    let mut generic_types = vec![];
    let mut tok = lex.next();
    while tok.is_type() {
        generic_types.push(lex.slice().to_string());
        tok = lex.next();
    }
    if tok != Some(Ok(Token::GreaterThan)) {
        return err(lex, "`>` after generic type declaration");
    }
    Ok(generic_types)
}

fn parse_assignment<const MUTABLE: bool>(
    lex: &mut Lexer<Token>,
    name: String,
) -> Result<Expr, String> {
    Ok(Expr::Assign {
        name,
        mutable: MUTABLE,
        value: Box::new(parse_expression(lex)?),
    })
}

fn parse_expression(lex: &mut Lexer<Token>) -> Result<Expr, String> {
    let lhs = parse_primary(lex)?;
    parse_binary_expression(lex, lhs, 1)
}

fn parse_primary(lex: &mut Lexer<Token>) -> Result<Expr, String> {
    if let Some(Ok(tok)) = lex.next() {
        match tok {
            Token::Identifier => {
                let name = lex.slice().to_string();
                let Some(Ok(tok)) = lex.peek() else {
                    return err(&lex, "token after identifier");
                };
                match tok {
                    Token::Apostrophe => {
                        lex.next();
                        parse_assignment::<false>(lex, name)
                    }
                    Token::Comma => {
                        lex.next();
                        parse_assignment::<true>(lex, name)
                    }
                    _ => parse_identifier(lex, name),
                }
            }
            Token::Float => parse_literal(lex, &tok),
            Token::Integer => parse_literal(lex, &tok),
            Token::String => parse_literal(lex, &tok),
            Token::LeftParen => {
                let expr = parse_expression(lex)?;
                if lex.next() != Some(Ok(Token::RightParen)) {
                    return err(lex, "closing parenthesis `)`");
                }
                Ok(expr)
            }
            Token::LeftBracket => {
                let mut elements = vec![];
                while lex.peek() != Some(Ok(Token::RightBracket)) {
                    elements.push(parse_expression(lex)?);
                }
                lex.next();
                Ok(Expr::List(elements))
            }
            Token::Comment => parse_primary(lex),
            _ => err(lex, "primary expression"),
        }
    } else {
        err(lex, "primary expression")
    }
}

fn parse_type(lex: &mut Lexer<Token>, generic_types: &Vec<String>) -> Result<Type, String> {
    if lex.slice() == "[" {
        if !lex.next().is_type() {
            return err(lex, "type after `[`");
        }
        let inner_type = parse_type(lex, generic_types)?;
        if lex.next() != Some(Ok(Token::RightBracket)) {
            return err(lex, "closing bracket `]`");
        }
        return Ok(Type::List(Box::new(inner_type)));
    }
    let result = Ok(match lex.slice() {
        "Int" => Type::Integer,
        "Float" => Type::Float,
        "String" => Type::String,
        tok if generic_types.contains(&tok.to_string()) => Type::Generic(tok.to_string()),
        _ => return err(lex, "type"),
    });
    if lex.clone().next() == Some(Ok(Token::LeftBracket)) {
        lex.next();
        if !lex.next().is_integer() {
            return err(lex, "size of array after `[`");
        }
        let size = lex.slice().parse::<usize>().unwrap();
        if lex.next() != Some(Ok(Token::RightBracket)) {
            return err(lex, "closing bracket `]` for array");
        }
        return Ok(Type::Array(Box::new(result?), size));
    }
    result
}

fn parse_parameter(lex: &mut Lexer<Token>) -> Result<Pattern, String> {
    let Some(Ok(tok)) = lex.next() else {
        return err(lex, "literal function parameter");
    };
    match tok {
        Token::Float => {
            let value = lex.slice().parse::<f64>().unwrap();
            Ok(Pattern::Literal(Literal::Float(value)))
        }
        Token::Integer => {
            let value = lex.slice().parse::<i64>().unwrap();
            Ok(Pattern::Literal(Literal::Integer(value)))
        }
        Token::String => {
            let value = lex.slice().trim_matches('"').to_string();
            Ok(Pattern::Literal(Literal::String(value)))
        }
        Token::Identifier => Ok(Pattern::Variable(lex.slice().to_string())),
        Token::Underscore => Ok(Pattern::Wildcard),
        Token::LeftBracket => {
            let mut elements = vec![];
            while lex.peek() != Some(Ok(Token::RightBracket)) {
                elements.push(parse_parameter(lex)?);
            }
            lex.next();
            Ok(Pattern::List(elements))
        }
        _ => err(lex, "literal function parameter"),
    }
}

fn parse_literal(lex: &mut Lexer<Token>, tok: &Token) -> Result<Expr, String> {
    match tok {
        Token::Float => {
            let value = lex.slice().parse::<f64>().unwrap();
            Ok(Expr::Literal(Literal::Float(value)))
        }
        Token::Integer => {
            let value = lex.slice().parse::<i64>().unwrap();
            Ok(Expr::Literal(Literal::Integer(value)))
        }
        Token::String => {
            let value = lex.slice().trim_matches('"').to_string();
            Ok(Expr::Literal(Literal::String(value)))
        }
        _ => err(lex, "literal expression"),
    }
}

fn parse_identifier(lex: &mut Lexer<Token>, name: String) -> Result<Expr, String> {
    if lex.peek() == Some(Ok(Token::LeftParen)) {
        lex.next();
        let mut args = vec![];
        while lex.peek() != Some(Ok(Token::RightParen)) {
            args.push(parse_expression(lex)?);
        }
        lex.next();
        Ok(Expr::Call { callee: name, args })
    } else {
        Ok(Expr::Variable(name))
    }
}

fn parse_binary_expression(
    lex: &mut Lexer<Token>,
    mut lhs: Expr,
    precedence: u8,
) -> Result<Expr, String> {
    while let Some(Ok(tok)) = lex.peek() {
        let prec = tok.get_precedence();
        if prec < precedence {
            return Ok(lhs);
        }
        lex.next();
        let op = lex.slice().to_string();
        let mut rhs = parse_primary(lex)?;
        if let Some(Ok(next_tok)) = lex.peek() {
            let next_prec = next_tok.get_precedence();
            if prec <= next_prec {
                let is_left_associative = if !matches!(op.as_str(), "<<" | "<|" | "**") {
                    1
                } else {
                    0
                };
                rhs = parse_binary_expression(lex, rhs, next_prec + is_left_associative)?;
            }
        }
        lhs = Expr::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    Ok(lhs)
}

fn err<T>(lex: &Lexer<Token>, expect: &str) -> Result<T, String> {
    Err(format!(
        "Expected {}, but got `{}` at {:?}.",
        expect,
        lex.slice(),
        lex.span()
    ))
}
