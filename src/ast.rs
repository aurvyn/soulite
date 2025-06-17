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
                    let cout = lhs.to_rust();
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
                            "write!({}, {}).unwrap();\n\t{}",
                            cout,
                            inner_lhs.to_rust(),
                            rhs.to_rust()
                        )
                    } else {
                        format!("write!({}, {}).unwrap()", cout, rhs.to_rust())
                    }
                }
                _ => format!("{} {} {}", lhs.to_rust(), op, rhs.to_rust()),
            },
            Expr::Call { callee, args } => format!(
                "{}({})",
                callee,
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
    // only support one equation for now
    fn to_rust(&self) -> String {
        if self.equations.is_empty() {
            return String::new();
        }
        let equation = &self.equations.last().unwrap();
        let signature = &self.signature;
        let param = signature
            .arg_types
            .iter()
            .zip(equation.parameters_list.iter())
            .map(|(t, param)| format!("{}: {}", param.to_rust(), t.to_rust()))
            .collect::<Vec<_>>()
            .join(", ");
        let head = format!(
            "fn {}({}) -> ({})",
            signature.name,
            param,
            signature
                .return_types
                .iter()
                .map(|t| t.to_rust())
                .collect::<Vec<_>>()
                .join(", ")
        );
        let body = equation
            .body
            .iter()
            .map(|expr| expr.to_rust())
            .collect::<Vec<_>>()
            .join(";\n\t");
        format!("{} {{\n\t{}\n}}", head, body)
    }
}

pub struct Import {
    pub filename: String,
    pub items: Vec<String>,
}

impl ToRust for Import {
    fn to_rust(&self) -> String {
        let module = format!("mod {};", self.filename);
        if self.items.is_empty() {
            module
        } else {
            let items = self.items.join(", ");
            format!("{}\nuse {}::{{{}}};", module, self.filename, items)
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
        format!("{}\n\n{}\n\n{}\n", imports, variables, functions)
    }
}
