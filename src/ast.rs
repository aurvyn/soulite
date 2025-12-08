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

trait JoinToRust {
    fn to_rust(&self, sep: &str) -> String;
}

impl<T: ToRust> JoinToRust for Vec<T> {
    fn to_rust(&self, sep: &str) -> String {
        self.iter()
            .map(|item| item.to_rust())
            .collect::<Vec<_>>()
            .join(sep)
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
            Pattern::List(patterns) => format!("[{}]", patterns.to_rust(",")),
            Pattern::Wildcard => "_".to_string(),
        }
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
    This,
    None,
    Reference(Box<Expr>),
    List(Vec<Expr>),
    Literal(Literal),
    Variable(String),
    Some(Box<Expr>),
    Ok(Box<Expr>),
    Err(Box<Expr>),
    Binary {
        op: String,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Ternary {
        condition: Box<Expr>,
        if_true: Box<Expr>,
        if_false: Box<Expr>,
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
            Expr::This => String::from("Self"),
            Expr::None => String::from("Option<_>"),
            Expr::Reference(inner) => format!("&{}", inner.to_rust_type()),
            Expr::List(items) => format!(
                "Vec<{}>",
                items
                    .first()
                    .map_or("()".to_string(), |item| item.to_rust_type())
            ),
            Expr::Literal(lit) => lit.to_rust_type(),
            Expr::Variable(_) => String::from("_"),
            Expr::Some(expr) => format!("Option<{}>", expr.to_rust_type()),
            Expr::Ok(expr) | Expr::Err(expr) => format!("Result<{}>", expr.to_rust_type()),
            Expr::Binary { op: _, lhs, rhs: _ } => lhs.to_rust_type(),
            Expr::Ternary {
                condition: _,
                if_true,
                if_false: _,
            } => if_true.to_rust_type(),
            Expr::Call { callee: _, args: _ } => String::from("_"),
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
            Expr::This => String::from("self"),
            Expr::None => String::from("None"),
            Expr::Reference(inner) => format!("&{}", inner.to_rust()),
            Expr::List(items) => format!("vec![{}]", items.to_rust(",")),
            Expr::Literal(lit) => lit.to_rust(),
            Expr::Variable(name) => name.to_rust(),
            Expr::Some(expr) => format!("Some({})", expr.to_rust()),
            Expr::Ok(expr) => format!("Ok({})", expr.to_rust()),
            Expr::Err(expr) => format!("Err({})", expr.to_rust()),
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
            Expr::Ternary {
                condition,
                if_true,
                if_false,
            } => format!(
                "if {} {{{}}} else {{{}}}",
                condition.to_rust(),
                if_true.to_rust(),
                if_false.to_rust()
            ),
            Expr::Call { callee, args } => format!(
                "{}({}{})",
                if callee == "main" { "start" } else { &callee },
                if callee == "join" { "&" } else { "" },
                args.to_rust(",")
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
                    "{} {}{}:{}={};",
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

pub enum Type {
    Integer,
    Float,
    String,
    Reference(Box<Type>),
    List(Box<Type>),
    Array(Box<Type>, usize),
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
    Generic(String),
}

impl ToRust for Type {
    fn to_rust(&self) -> String {
        match self {
            Type::Integer => String::from("i64"),
            Type::Float => String::from("f64"),
            Type::String => String::from("String"),
            Type::Reference(inner) => format!("&{}", inner.to_rust()),
            Type::List(inner) => format!("Vec<{}>", inner.to_rust()),
            Type::Array(inner, size) => format!("[{};{}]", inner.to_rust(), size),
            Type::Option(inner) => format!("Option<{}>", inner.to_rust()),
            Type::Result(inner, err) => format!("Result<{},{}>", inner.to_rust(), err.to_rust()),
            Type::Generic(name) => name.to_rust(),
        }
    }
}

pub struct TypeSignature {
    pub name: String,
    pub arg_types: Vec<Type>,
    pub return_types: Vec<Type>,
    pub is_method: bool,
}

impl ToRust for TypeSignature {
    fn to_rust(&self) -> String {
        format!(
            "fn {}({}{}) -> {};",
            self.name.to_rust(),
            if self.is_method { "&mut self," } else { "" },
            self.arg_types
                .iter()
                .map(|t| format!("_: {}", t.to_rust()))
                .collect::<Vec<_>>()
                .join(","),
            match self.return_types.len() {
                0 => "()".to_string(),
                1 => self.return_types.to_rust(","),
                _ => format!("({})", self.return_types.to_rust(",")),
            }
        )
    }
}

pub struct Equation {
    pub parameters_list: Vec<Pattern>,
    pub guard: Option<Expr>,
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
        let mut head = format!(
            "fn {}({}{})",
            func_name,
            if self.signature.is_method {
                "&mut self,"
            } else {
                ""
            },
            param.join(",")
        );
        let ret = signature.return_types.to_rust(",");
        match signature.return_types.len() {
            0 => {}
            1 => write!(head, " -> {}", ret).unwrap(),
            _ => write!(head, " -> ({})", ret).unwrap(),
        }
        let body = if matcher.len() == 0 {
            equation.body.to_rust(";")
        } else {
            let mut content = String::new();
            for equation in self.equations.iter().take(self.equations.len() - 1) {
                let mut matching = equation.parameters_list.to_rust(",");
                if matcher.len() > 1 {
                    matching = format!("({})", matching);
                }
                if let Some(cond) = &equation.guard {
                    matching = format!("{} if {}", matching, cond.to_rust());
                }
                write!(content, "{}=>{{{}}}", matching, equation.body.to_rust(";")).unwrap();
            }
            write!(content, "_=>{{{}}}", equation.body.to_rust(";")).unwrap();
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

pub struct Implementation {
    pub struct_name: String,
    pub trait_name: String,
    pub generic_types: Vec<String>,
    pub methods: Vec<Function>,
}

impl ToRust for Implementation {
    fn to_rust(&self) -> String {
        let methods = self.methods.to_rust("");
        let generic_types = if self.generic_types.is_empty() {
            String::new()
        } else {
            format!("<{}>", self.generic_types.to_rust(","))
        };
        format!(
            "impl{} {} for {}{} {{{}}}",
            generic_types,
            self.trait_name.to_rust(),
            self.struct_name.to_rust(),
            generic_types,
            methods
        )
    }
}

type Field = (String, Type);

pub struct Struct {
    pub name: String,
    pub generic_types: Vec<String>,
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
        let name = self.name.to_rust();
        let generic_types = if self.generic_types.is_empty() {
            String::new()
        } else {
            format!("<{}>", self.generic_types.to_rust(","))
        };
        let base = format!("struct {}{} {{{}}}", name, generic_types, fields);
        if self.methods.is_empty() {
            return base;
        }
        format!(
            "{} impl{} {}{} {{{}}}",
            base,
            generic_types,
            name,
            generic_types,
            self.methods.to_rust("")
        )
    }
}

pub struct Trait {
    pub name: String,
    pub generic_types: Vec<String>,
    pub signatures: Vec<TypeSignature>,
}

impl ToRust for Trait {
    fn to_rust(&self) -> String {
        let signatures = self.signatures.to_rust("");
        let generic_types = if self.generic_types.is_empty() {
            String::new()
        } else {
            format!("<{}>", self.generic_types.to_rust(","))
        };
        format!(
            "trait {}{} {{{}}}",
            self.name.to_rust(),
            generic_types,
            signatures
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
    pub traits: Vec<Trait>,
    pub structs: Vec<Struct>,
    pub impls: Vec<Implementation>,
    pub functions: Vec<Function>,
    pub variables: Vec<Expr>,
}

impl ToRust for Program {
    fn to_rust(&self) -> String {
        let mut out = String::new();
        let all = self
            .imports
            .iter()
            .map(|i| i as &dyn ToRust)
            .chain(self.traits.iter().map(|t| t as &dyn ToRust))
            .chain(self.structs.iter().map(|s| s as &dyn ToRust))
            .chain(self.impls.iter().map(|i| i as &dyn ToRust))
            .chain(self.variables.iter().map(|v| v as &dyn ToRust))
            .chain(self.functions.iter().map(|f| f as &dyn ToRust));
        for item in all {
            out.push_str(&item.to_rust());
        }
        out.push_str(
            if self.functions.iter().any(|f| f.signature.name == "main") {
                "fn main() {start(std::env::args().skip(1).collect())}"
            } else {
                "fn main() {}"
            },
        );
        out
    }
}
