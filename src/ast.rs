pub trait ToRust {
    fn to_rust(&self) -> String;
}

impl ToRust for String {
    fn to_rust(&self) -> String {
        match self.as_str() {
            "abstract" | "as" | "async" | "await" | "become" | "box" | "break" | "const"
            | "continue" | "crate" | "do" | "dyn" | "else" | "enum" | "extern" | "false"
            | "final" | "fn" | "for" | "gen" | "if" | "impl" | "in" | "let" | "loop" | "macro"
            | "match" | "mod" | "move" | "mut" | "override" | "priv" | "pub" | "ref" | "return"
            | "self" | "Self" | "static" | "struct" | "super" | "trait" | "true" | "try"
            | "type" | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while"
            | "yield" => format!("_{}", self),
            _ => self.to_owned(),
        }
    }
}

#[derive(Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
}

impl ToRust for Literal {
    fn to_rust(&self) -> String {
        match self {
            Literal::Integer(i) => i.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::String(s) => format!("format!(\"{}\")", s),
        }
    }
}

pub enum Pattern {
    Literal(Literal),
    Variable(String),
    List(Vec<Pattern>),
    Wildcard,
}

impl ToRust for Pattern {
    fn to_rust(&self) -> String {
        match self {
            Pattern::Literal(Literal::String(lit)) => format!("\"{}\"", lit),
            Pattern::Literal(lit) => lit.to_rust(),
            Pattern::Variable(name) => name.to_rust(),
            Pattern::List(patterns) => format!(
                "[{}]",
                patterns
                    .iter()
                    .map(|p| p.to_rust())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Pattern::Wildcard => "_".to_string(),
        }
    }
}

#[derive(Clone)]
pub enum Expr {
    List(Vec<Expr>),
    Literal(Literal),
    Variable(String),
    Binary {
        op: String,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    Assign {
        name: String,
        mutable: bool,
        value: Box<Expr>,
    },
}

impl ToRust for Expr {
    fn to_rust(&self) -> String {
        match self {
            Expr::List(items) => format!(
                "vec![{}]",
                items
                    .iter()
                    .map(|item| item.to_rust())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expr::Literal(lit) => lit.to_rust(),
            Expr::Variable(name) => name.to_rust(),
            Expr::Binary { op, lhs, rhs } => match op.as_str() {
                "<<" | "<|" => {
                    let write_func = if op == "<<" { "" } else { "ln" };
                    if let Expr::Binary {
                        lhs: inner_lhs,
                        rhs: inner_rhs,
                        op,
                    } = &**rhs
                    {
                        let rhs = Expr::Binary {
                            lhs: lhs.clone(),
                            op: op.clone(),
                            rhs: inner_rhs.clone(),
                        };
                        format!(
                            "write{}!(stdout(), \"{{}}\", {}).unwrap();\n\t{}",
                            write_func,
                            inner_lhs.to_rust(),
                            rhs.to_rust()
                        )
                    } else {
                        format!(
                            "write{}!(stdout(), \"{{}}\", {}).unwrap()",
                            write_func,
                            rhs.to_rust()
                        )
                    }
                }
                _ => format!("{}{}{}", lhs.to_rust(), op, rhs.to_rust()),
            },
            Expr::Call { callee, args } => format!(
                "{}({}{})",
                if callee == "main" { "start" } else { &callee },
                if callee == "join" { "&" } else { "" },
                args.iter()
                    .map(|arg| arg.to_rust())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expr::Assign {
                name,
                mutable,
                value,
            } => format!(
                "let {}{} = {}",
                if *mutable { "mut " } else { "" },
                name,
                value.to_rust()
            ),
        }
    }
}

pub enum Type {
    Integer,
    Float,
    String,
    List(Box<Type>),
}

impl ToRust for Type {
    fn to_rust(&self) -> String {
        match self {
            Type::Integer => "i64".to_string(),
            Type::Float => "f64".to_string(),
            Type::String => "String".to_string(),
            Type::List(inner) => format!("Vec<{}>", inner.to_rust()),
        }
    }
}

pub struct TypeSignature {
    pub name: String,
    pub arg_types: Vec<Type>,
    pub return_types: Vec<Type>,
}

pub struct Equation {
    pub parameters_list: Vec<Pattern>,
    pub body: Vec<Expr>,
}

pub struct Function {
    pub signature: TypeSignature,
    pub equations: Vec<Equation>,
}

impl ToRust for Function {
    fn to_rust(&self) -> String {
        if self.equations.is_empty() {
            return String::new();
        }
        let equation = &self.equations.last().unwrap();
        let signature = &self.signature;
        let func_name = if signature.name == "main" {
            "start"
        } else {
            &signature.name
        };
        let (param, matcher): (Vec<_>, Vec<_>) = signature
            .arg_types
            .iter()
            .zip(equation.parameters_list.iter())
            .map(|(t, param)| {
                let matcher = match t {
                    Type::String => format!("{}.as_str()", param.to_rust()),
                    Type::List(_) => format!(
                        "{}.iter().map(String::as_str).collect::<Vec<_>>()[..]",
                        param.to_rust()
                    ),
                    _ => param.to_rust(),
                };
                (format!("{}: {}", param.to_rust(), t.to_rust()), matcher)
            })
            .unzip();
        let mut head = format!("fn {}({})", func_name, param.join(", "));
        match signature.return_types.len() {
            0 => (),
            1 => head.push_str(&format!(" -> {}", signature.return_types[0].to_rust())),
            _ => {
                let return_types = signature
                    .return_types
                    .iter()
                    .map(|t| t.to_rust())
                    .collect::<Vec<_>>()
                    .join(", ");
                head.push_str(&format!(" -> ({})", return_types));
            }
        }
        let body = if matcher.len() == 0 {
            equation
                .body
                .iter()
                .map(|expr| expr.to_rust())
                .collect::<Vec<_>>()
                .join(";\n\t")
        } else {
            let mut content = String::new();
            for equation in self.equations.iter().take(self.equations.len() - 1) {
                let matching = equation
                    .parameters_list
                    .iter()
                    .map(|p| p.to_rust())
                    .collect::<Vec<_>>()
                    .join(", ");
                content.push_str(&format!(
                    "{} => {{\n\t{}\n}}",
                    if matcher.len() == 1 {
                        matching
                    } else {
                        format!("({})", matching)
                    },
                    equation
                        .body
                        .iter()
                        .map(|expr| expr.to_rust())
                        .collect::<Vec<_>>()
                        .join(";\n\t")
                ));
            }
            content.push_str(&format!(
                "_ => {{\n\t{}\n}}",
                equation
                    .body
                    .iter()
                    .map(|expr| expr.to_rust())
                    .collect::<Vec<_>>()
                    .join(";\n\t")
            ));
            content
        };
        format!(
            "{} {{{}}}",
            head,
            match matcher.len() {
                0 => format!("{}", body),
                1 => format!("match {} {{\n{}\n}}", matcher.join(", "), body),
                _ => format!("match ({}) {{\n{}\n}}", matcher.join(", "), body),
            }
        )
    }
}

pub struct Import {
    pub filename: String,
    pub items: Vec<String>,
}

impl ToRust for Import {
    fn to_rust(&self) -> String {
        if self.filename == "std" {
            let mut import = String::new();
            for item in &self.items {
                match item.as_str() {
                    "cout" => import.push_str("\tio::{Write, stdout},\n"),
                    _ => (),
                }
            }
            format!("use std::{{\n{}}};", import)
        } else {
            let module = format!("mod {};", self.filename);
            if self.items.is_empty() {
                module
            } else {
                let items = self.items.join(", ");
                format!("{}\nuse {}::{{{}}};", module, self.filename, items)
            }
        }
    }
}

pub struct Program {
    pub imports: Vec<Import>,
    pub functions: Vec<Function>,
    pub variables: Vec<Expr>,
}

impl ToRust for Program {
    fn to_rust(&self) -> String {
        let imports = self
            .imports
            .iter()
            .map(|i| i.to_rust())
            .collect::<Vec<_>>()
            .join("\n");
        let functions = self
            .functions
            .iter()
            .map(|f| f.to_rust())
            .collect::<Vec<_>>()
            .join("\n\n");
        let variables = self
            .variables
            .iter()
            .map(|v| v.to_rust())
            .collect::<Vec<_>>()
            .join(";\n");
        format!(
            "{}\n\n{}\n\n{}\n\nfn main() {{\n\tstart(std::env::args().skip(1).collect())\n}}\n",
            imports, variables, functions
        )
    }
}
