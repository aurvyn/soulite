use std::fmt::Write;

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

impl Literal {
    fn to_rust_type(&self) -> String {
        match self {
            Literal::Integer(_) => "i64".to_string(),
            Literal::Float(_) => "f64".to_string(),
            Literal::String(_) => "String".to_string(),
        }
    }
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

impl ToRust for Vec<Pattern> {
    fn to_rust(&self) -> String {
        self.iter()
            .map(|pattern| pattern.to_rust())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[derive(Clone)]
pub enum AssignType {
    Const,
    Static,
    Normal,
}

impl ToRust for AssignType {
    fn to_rust(&self) -> String {
        match self {
            AssignType::Const => "const",
            AssignType::Static => "static",
            AssignType::Normal => "let",
        }
        .to_string()
    }
}

#[derive(Clone)]
pub enum Expr {
    Reference(Box<Expr>),
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
        assign_type: AssignType,
        mutable: bool,
        value: Box<Expr>,
    },
}

impl Expr {
    fn to_rust_type(&self) -> String {
        match self {
            Expr::Reference(inner) => format!("&{}", inner.to_rust_type()),
            Expr::List(items) => format!(
                "Vec<{}>",
                items
                    .first()
                    .map_or("()".to_string(), |item| item.to_rust_type())
            ),
            Expr::Literal(lit) => lit.to_rust_type(),
            Expr::Variable(_) => "_".to_string(),
            Expr::Binary { op: _, lhs, rhs: _ } => lhs.to_rust_type(),
            Expr::Call { callee: _, args: _ } => "_".to_string(),
            Expr::Assign {
                name: _,
                assign_type: _,
                mutable: _,
                value,
            } => value.to_rust_type(),
        }
    }
}

impl ToRust for Expr {
    fn to_rust(&self) -> String {
        match self {
            Expr::Reference(inner) => format!("&{}", inner.to_rust()),
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
                            "write{}!(stdout(), \"{{}}\", {}).unwrap();{}",
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
                assign_type,
                value,
            } => {
                let (t, val) = if matches!(assign_type, AssignType::Const | AssignType::Static)
                    && let Expr::Literal(Literal::String(val)) = *value.clone()
                {
                    ("&str".to_string(), format!("\"{}\"", val))
                } else {
                    (value.to_rust_type(), value.to_rust())
                };
                format!(
                    "{} {}{}:{}={}",
                    assign_type.to_rust(),
                    if *mutable { "mut " } else { "" },
                    name,
                    t,
                    val
                )
            }
        }
    }
}

impl ToRust for Vec<Expr> {
    fn to_rust(&self) -> String {
        self.iter()
            .map(|expr| expr.to_rust())
            .collect::<Vec<_>>()
            .join("; ")
    }
}

pub enum Type {
    Integer,
    Float,
    String,
    Reference(Box<Type>),
    List(Box<Type>),
    Array(Box<Type>, usize),
    Generic(String),
}

impl ToRust for Type {
    fn to_rust(&self) -> String {
        match self {
            Type::Integer => "i64".to_string(),
            Type::Float => "f64".to_string(),
            Type::String => "String".to_string(),
            Type::Reference(inner) => format!("&{}", inner.to_rust()),
            Type::List(inner) => format!("Vec<{}>", inner.to_rust()),
            Type::Array(inner, size) => format!("[{}; {}]", inner.to_rust(), size),
            Type::Generic(name) => name.to_string(),
        }
    }
}

impl ToRust for Vec<Type> {
    fn to_rust(&self) -> String {
        self.iter()
            .map(|t| t.to_rust())
            .collect::<Vec<_>>()
            .join(", ")
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
        let ret = signature.return_types.to_rust();
        match signature.return_types.len() {
            0 => {}
            1 => write!(head, " -> {}", ret).unwrap(),
            _ => write!(head, " -> ({})", ret).unwrap(),
        }
        let body = if matcher.len() == 0 {
            equation.body.to_rust()
        } else {
            let mut content = String::new();
            for equation in self.equations.iter().take(self.equations.len() - 1) {
                let mut matching = equation.parameters_list.to_rust();
                if matcher.len() > 1 {
                    matching = format!("({})", matching);
                }
                write!(content, "{} => {{{}}}", matching, equation.body.to_rust()).unwrap();
            }
            write!(content, "_ => {{{}}}", equation.body.to_rust()).unwrap();
            content
        };
        format!(
            "{} {{{}}}",
            head,
            match matcher.len() {
                0 => format!("{}", body),
                1 => format!("match {} {{{}}}", matcher.join(", "), body),
                _ => format!("match ({}) {{{}}}", matcher.join(", "), body),
            }
        )
    }
}

type Field = (String, Type);

pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub methods: Vec<Function>,
}

impl ToRust for Struct {
    fn to_rust(&self) -> String {
        let fields = self
            .fields
            .iter()
            .map(|(name, kind)| format!("{}: {}", name.to_rust(), kind.to_rust()))
            .collect::<Vec<_>>()
            .join(", ");
        format!("struct {} {{ {} }}", self.name.to_rust(), fields)
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
                    "cout" => import.push_str("io::{Write, stdout},"),
                    _ => (),
                }
            }
            format!("use std::{{{}}};", import)
        } else {
            let module = format!("mod {};", self.filename);
            if self.items.is_empty() {
                module
            } else {
                let items = self.items.join(", ");
                format!("{}use {}::{{{}}};", module, self.filename, items)
            }
        }
    }
}

pub struct Program {
    pub imports: Vec<Import>,
    pub structs: Vec<Struct>,
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
            .join("");
        let structs = self
            .structs
            .iter()
            .map(|s| s.to_rust())
            .collect::<Vec<_>>()
            .join("");
        let functions = self
            .functions
            .iter()
            .map(|f| f.to_rust())
            .collect::<Vec<_>>()
            .join("");
        let variables = self
            .variables
            .iter()
            .map(|v| format!("{};", v.to_rust()))
            .collect::<Vec<_>>()
            .join("");
        format!(
            "{}{}{}{}{}",
            imports,
            structs,
            variables,
            functions,
            if self.functions.iter().any(|f| f.signature.name == "main") {
                "fn main() {start(std::env::args().skip(1).collect())}"
            } else {
                "fn main() {}"
            }
        )
    }
}
