pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::Integer(i) => i.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
        }
    }
}

pub enum Expr {
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

pub trait ToRust {
    fn to_rust(&self) -> String;
}

impl ToRust for Expr {
    fn to_rust(&self) -> String {
        match self {
            Expr::Literal(lit) => lit.to_string(),
            Expr::Variable(name) => name.clone(),
            Expr::Binary { op, lhs, rhs } =>
                format!("{} {} {}", lhs.to_rust(), op, rhs.to_rust()),
            Expr::Call { callee, args } =>
                format!("{}({})", callee, args.iter()
                    .map(|arg| arg.to_rust())
                    .collect::<Vec<_>>()
                    .join(", ")),
            Expr::Assign { name, mutable, value } =>
                format!("let {}{} = {};", if *mutable { "mut " } else { "" }, name, value.to_rust()),
        }
    }
}

pub struct TypeSignature {
    pub name: String,
    pub arg_types: Vec<String>,
    pub return_type: Vec<String>,
}

pub struct Equation {
    pub parameters_list: Vec<Expr>,
    pub body: Vec<Expr>,
}

pub struct Function {
    pub signature: TypeSignature,
    pub equations: Vec<Equation>,
}

impl Function {
    // only support one equation for now
    pub fn to_rust(&self) -> String {
        if self.equations.is_empty() {
            return String::new();
        }
        let equation = &self.equations[0];
        let signature = &self.signature;
        let param = signature.arg_types.iter()
            .zip(equation.parameters_list.iter())
            .map(|(t, param)| format!("{}: {}", param.to_rust(), t))
            .collect::<Vec<_>>()
            .join(", ");
        let head = format!("fn {}({}) -> ({})", signature.name, param, signature.return_type.join(", "));
        let body = equation.body.iter()
            .map(|expr| expr.to_rust())
            .collect::<Vec<_>>()
            .join("\n    ");
        format!("{} {{\n    {}\n}}", head, body)
    }
}

pub struct Import {
    pub filename: String,
    pub items: Vec<String>,
}

impl Import {
    pub fn to_rust(&self) -> String {
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

impl Program {
    pub fn to_rust(&self) -> String {
        let imports = self.imports.iter()
            .map(|i| i.to_rust())
            .collect::<Vec<_>>()
            .join("\n");
        let functions = self.functions.iter()
            .map(|f| f.to_rust())
            .collect::<Vec<_>>()
            .join("\n\n\t");
        let variables = self.variables.iter()
            .map(|v| v.to_rust())
            .collect::<Vec<_>>()
            .join("\n\t");
        format!("{}\n\nfn main() {{\n\t{}\n\n\t{}\n}}", imports, variables, functions)
    }
}
