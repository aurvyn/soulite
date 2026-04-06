use std::vec;

use crate::{
    ast::{
        AssignType, Expr, Function, Implementation, Import, Literal, Pattern, Program, Struct,
        Trait, Type, TypeSignature,
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
        traits: vec![],
        structs: vec![],
        impls: vec![],
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
                let (param_names, mut tok) = parse_params(&mut lex);
                if tok.is_colon() {
                    tok = lex.peek();
                    if tok.is_arrow() || tok.is_type() {
                        program.functions.push(parse_function(
                            &mut lex,
                            name,
                            param_names,
                            &vec![],
                            false,
                            1,
                        )?)
                    } else {
                        program.variables.push(parse_assignment(
                            &mut lex,
                            name,
                            AssignType::Static,
                        )?)
                    }
                } else {
                    return err(&lex, "variable or function marker");
                }
            }
            Token::Type => {
                let name = lex.slice().to_string();
                let mut tok = lex.next();
                let generic_types = if tok == Some(Ok(Token::LessThan)) {
                    let result = parse_generic_types(&mut lex)?;
                    tok = lex.next();
                    result
                } else {
                    vec![]
                };
                match tok {
                    Some(Ok(Token::FatArrow)) => {
                        program
                            .impls
                            .push(parse_impl(&mut lex, name, generic_types)?)
                    }
                    Some(Ok(Token::Assign)) => {
                        program
                            .structs
                            .push(parse_struct(&mut lex, name, generic_types)?)
                    }
                    Some(Ok(Token::Colon)) => {
                        program
                            .traits
                            .push(parse_trait(&mut lex, name, generic_types)?)
                    }
                    _ => return err(&lex, "colon, arrow, or generic type after struct name"),
                }
            }
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

fn parse_trait(
    lex: &mut Lexer<Token>,
    name: String,
    generics: Vec<String>,
) -> Result<Trait, String> {
    let mut signatures = vec![];
    if !lex.next().is_newline() {
        return err(lex, "newline after `:`");
    }
    while lex.peek().is_tab() {
        lex.next();
        if !lex.next().is_identifier() {
            return err(&lex, "method name after tab");
        }
        let method_name = lex.slice().to_string();
        let (param_names, tok) = parse_params(lex);
        if !tok.is_colon() {
            return err(&lex, "`:` for method");
        }
        signatures.push(parse_signature(
            lex,
            method_name,
            param_names,
            &generics,
            true,
        )?);
    }
    Ok(Trait {
        name,
        generics,
        signatures,
    })
}

fn parse_struct(
    lex: &mut Lexer<Token>,
    name: String,
    generics: Vec<String>,
) -> Result<Struct, String> {
    let mut fields = vec![];
    let mut methods = vec![];
    while lex.peek().is_newline() && lex.lookahead().is_tab() {
        lex.step();
        if !lex.next().is_identifier() {
            return err(&lex, "field name after tab");
        }
        let field_name = lex.slice().to_string();
        let (param_names, tok) = parse_params(lex);
        if tok.is_colon() {
            methods.push(parse_function(
                lex,
                field_name,
                param_names,
                &generics,
                true,
                2,
            )?)
        } else if tok.is_type() {
            fields.push((field_name, parse_type(lex, &generics)?));
        } else {
            return err(&lex, "field type or `:` for method");
        }
    }
    Ok(Struct {
        name,
        generics,
        fields,
        methods,
    })
}

fn parse_impl(
    lex: &mut Lexer<Token>,
    struct_name: String,
    generic_types: Vec<String>,
) -> Result<Implementation, String> {
    if !lex.next().is_type() {
        return err(lex, "trait name after `=>`");
    }
    let trait_name = lex.slice().to_string();
    let mut methods = vec![];
    while lex.peek().is_newline() && lex.lookahead().is_tab() {
        lex.step();
        if !lex.next().is_identifier() {
            return err(&lex, "method name after tab");
        }
        let method_name = lex.slice().to_string();
        let (param_names, tok) = parse_params(lex);
        if !tok.is_colon() {
            return err(&lex, "`:` for method");
        }
        methods.push(parse_function(
            lex,
            method_name,
            param_names,
            &generic_types,
            true,
            2,
        )?);
    }
    Ok(Implementation {
        struct_name,
        trait_name,
        generic_types,
        methods,
    })
}

fn parse_params(lex: &mut Lexer<Token>) -> (Vec<String>, Option<Result<Token, ()>>) {
    let mut param_names = vec![];
    let mut tok = lex.next();
    while tok.is_identifier() {
        param_names.push(lex.slice().to_string());
        tok = lex.next();
    }
    (param_names, tok)
}

fn parse_signature(
    lex: &mut Lexer<Token>,
    name: String,
    param_names: Vec<String>,
    parent_generics: &Vec<String>,
    is_method: bool,
) -> Result<TypeSignature, String> {
    let mut signature = TypeSignature {
        name,
        param_names,
        param_types: vec![],
        generics: vec![],
        return_types: vec![],
        is_method,
    };
    let available_generics = parent_generics.clone();
    let mut tok = lex.next();
    while tok.is_type() {
        signature
            .param_types
            .push(parse_type(lex, &available_generics)?);
        tok = lex.next();
    }
    if !tok.is_newline() {
        if !tok.is_arrow() {
            return err(lex, "`->` after argument types");
        }
        while lex.next().is_type() {
            signature
                .return_types
                .push(parse_type(lex, &available_generics)?);
        }
    }
    Ok(signature)
}

fn parse_function(
    lex: &mut Lexer<Token>,
    name: String,
    param_names: Vec<String>,
    parent_generics: &Vec<String>,
    is_method: bool,
    indent: usize,
) -> Result<Function, String> {
    let mut func = Function {
        signature: parse_signature(lex, name, param_names, parent_generics, is_method)?,
        body: vec![],
    };
    let mut indents = indent;
    let mut tok;
    while indents >= indent {
        // TODO: need to handle indent level when returning, such as inside another function
        (indents, tok) = lex.skip_indents();
        if tok.is_newline() {
            lex.next();
            continue;
        }
        if indents > indent {
            return err(lex, "newline after empty line");
        }
        func.body.push(parse_expression(lex)?);
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

fn parse_assignment(
    lex: &mut Lexer<Token>,
    name: String,
    mut assign_type: AssignType,
) -> Result<Expr, String> {
    let Some(Ok(mut tok)) = lex.next() else {
        return err(lex, "expected token after colon");
    };
    let mut type_hint = None;
    if tok == Token::Type {
        type_hint = Some(parse_type(lex, &vec![])?);
        if let Some(Ok(token)) = lex.next() {
            tok = token;
        } else {
            return err(lex, "expected token after type");
        };
    }
    let mutable = match tok {
        Token::Colon => {
            assign_type = AssignType::Const;
            false
        }
        Token::Minus => false,
        Token::Assign => true,
        _ => return err(lex, "`-` or `=` after colon"),
    };
    Ok(Expr::Assign {
        name,
        assign_type,
        mutable,
        value: Box::new(parse_expression(lex)?),
        type_hint,
    })
}

fn parse_expression(lex: &mut Lexer<Token>) -> Result<Expr, String> {
    let lhs = if lex.peek() == Some(Ok(Token::Dot)) {
        Expr::This
    } else {
        parse_primary(lex)?
    };
    let expr = parse_binary_expression(lex, lhs, 1)?;
    parse_ternary_expression(lex, expr)
}

fn parse_primary(lex: &mut Lexer<Token>) -> Result<Expr, String> {
    let mut result = if let Some(Ok(tok)) = lex.next() {
        match tok {
            Token::Identifier => {
                let name = lex.slice().to_string();
                if lex.peek() != Some(Ok(Token::Colon)) {
                    parse_identifier(lex, name)?
                } else {
                    lex.next();
                    parse_assignment(lex, name, AssignType::Normal)?
                }
            }
            Token::ParamIdentifier => {
                let name = lex.slice()[1..].to_string();
                Expr::AnonParam(Box::new(parse_identifier(lex, name)?))
            }
            Token::Underscore => Expr::AnonParam(Box::new(Expr::None)),
            Token::Float | Token::Integer | Token::String => parse_literal(lex, &tok)?,
            Token::Star => {
                let expr = parse_expression(lex)?;
                Expr::Reference(Box::new(expr))
            }
            Token::LeftParen => {
                let expr = parse_expression(lex)?;
                if lex.next() != Some(Ok(Token::RightParen)) {
                    err(lex, "closing parenthesis `)`")?
                } else {
                    expr
                }
            }
            Token::LeftBracket => {
                let mut elements = vec![];
                while lex.peek() != Some(Ok(Token::RightBracket)) {
                    elements.push(parse_expression(lex)?);
                }
                lex.next();
                Expr::List(elements)
            }
            Token::LeftSome => {
                if lex.peek().is_right_paren() {
                    lex.next();
                    Expr::None
                } else {
                    let inner = parse_expression(lex)?;
                    if !lex.next().is_right_some() {
                        err(lex, "closing `|)` after Some inner expression")?
                    } else {
                        Expr::Some(Box::new(inner))
                    }
                }
            }
            Token::Comment => parse_primary(lex)?,
            _ => err(lex, "primary expression")?,
        }
    } else {
        err(lex, "primary expression")?
    };
    loop {
        match lex.peek() {
            Some(Ok(Token::Bang)) => {
                lex.next();
                result = Expr::Ok(Box::new(result));
            }
            Some(Ok(Token::Tick)) => {
                lex.next();
                result = Expr::Err(Box::new(result));
            }
            _ => break,
        }
    }
    Ok(result)
}

fn parse_num_type_bits(lex: &mut Lexer<Token>) -> Result<u8, String> {
    if lex.next() == Some(Ok(Token::Integer)) // currently `N 32` is valid
    && let Ok(bits) = lex.slice().parse::<u8>()
    && [8, 16, 32, 64, 128].contains(&bits)
    {
        Ok(bits)
    } else {
        err(lex, "8, 16, 32, 64, or 128 bits for this number type")
    }
}

fn parse_type(lex: &mut Lexer<Token>, generics: &Vec<String>) -> Result<Type, String> {
    let mut result = match lex.slice() {
        "(" => {
            let mut arg_types = vec![];
            let mut return_types = vec![];
            let mut tok = lex.next();
            while tok.is_type() {
                arg_types.push(parse_type(lex, generics)?);
                tok = lex.next();
            }
            if !tok.is_arrow() {
                return err(lex, "`->` after arg types");
            }
            tok = lex.next();
            while tok.is_type() {
                return_types.push(parse_type(lex, generics)?);
                tok = lex.next();
            }
            if !tok.is_right_paren() {
                return err(lex, "`)` after return types");
            }
            Type::Closure(arg_types, return_types)
        }
        "[" => {
            if !lex.next().is_type() {
                return err(lex, "type after `[`");
            }
            let inner_type = parse_type(lex, generics)?;
            if lex.next() != Some(Ok(Token::RightBracket)) {
                return err(lex, "closing bracket `]`");
            }
            Type::List(Box::new(inner_type))
        }
        "*" => {
            if !lex.next().is_type() {
                return err(lex, "type after `*`");
            }
            let inner_type = parse_type(lex, generics)?;
            Type::Reference(Box::new(inner_type))
        }
        _ => {
            let result = match lex.slice() {
                "N" => Type::Unsigned(parse_num_type_bits(lex)?),
                "Z" => Type::Integer(parse_num_type_bits(lex)?),
                "R" => Type::Float(parse_num_type_bits(lex)?),
                "String" => Type::String,
                tok if generics.contains(&tok.to_string()) => Type::Generic(tok.to_string()),
                _ => return err(lex, "type"),
            };
            if lex.peek() == Some(Ok(Token::LeftBracket)) {
                lex.next();
                if !lex.next().is_integer() {
                    return err(lex, "size of array after `[`");
                }
                let size = lex.slice().parse::<usize>().unwrap();
                if lex.next() != Some(Ok(Token::RightBracket)) {
                    return err(lex, "closing bracket `]` for array");
                }
                return Ok(Type::Array(Box::new(result), size));
            }
            result
        }
    };
    loop {
        match lex.peek() {
            Some(Ok(Token::Eroteme)) => {
                lex.next();
                result = Type::Option(Box::new(result));
            }
            Some(Ok(Token::Bang)) => {
                lex.next();
                lex.next();
                result = Type::Result(Box::new(result), Box::new(parse_type(lex, generics)?));
            }
            _ => break,
        }
    }
    Ok(result)
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
    Ok(if lex.peek() == Some(Ok(Token::LeftParen)) {
        lex.next();
        let mut args = vec![];
        while lex.peek() != Some(Ok(Token::RightParen)) {
            args.push(parse_expression(lex)?);
        }
        lex.next();
        Expr::Call { callee: name, args }
    } else {
        Expr::Variable(name)
    })
}

fn handle_anon_param(args: &mut Vec<String>, expr: &mut Expr) {
    if let Expr::AnonParam(param) = expr {
        let name = match &**param {
            Expr::Call { callee, .. } => callee.clone(),
            Expr::Variable(name) => name.clone(),
            Expr::None => {
                let name = format!("arg{}", args.len());
                **param = Expr::Variable(name.clone());
                name
            }
            _ => unreachable!(),
        };
        if !args.contains(&name) {
            args.push(name);
        }
    }
}

fn parse_binary_expression(
    lex: &mut Lexer<Token>,
    mut lhs: Expr,
    precedence: u8,
) -> Result<Expr, String> {
    while let Some(Ok(tok)) = lex.peek() {
        let mut args = vec![];
        handle_anon_param(&mut args, &mut lhs);
        let prec = tok.get_precedence();
        if prec < precedence {
            if let Expr::AnonParam(param) = lhs {
                lhs = Expr::Closure { args, body: param };
            }
            break;
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
        handle_anon_param(&mut args, &mut rhs);
        lhs = Expr::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
        if !args.is_empty() {
            lhs = Expr::Closure {
                args,
                body: Box::new(lhs),
            };
        }
    }
    Ok(lhs)
}

fn parse_ternary_expression(lex: &mut Lexer<Token>, mut expr: Expr) -> Result<Expr, String> {
    while lex.peek().is_if() {
        lex.next();
        let condition = parse_expression(lex)?;
        if !lex.next().is_semicolon() {
            return err(lex, "`;` after ternary condition");
        }
        let if_false = parse_expression(lex)?;
        expr = Expr::Ternary {
            condition: Box::new(condition),
            if_true: Box::new(expr),
            if_false: Box::new(if_false),
        };
    }
    Ok(expr)
}

fn err<T>(lex: &Lexer<Token>, expect: &str) -> Result<T, String> {
    Err(format!(
        "Expected {}, but got `{}` at {:?}.",
        expect,
        lex.slice(),
        lex.span()
    ))
}
