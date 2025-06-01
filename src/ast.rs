trait ExprAST {
    fn to_rust(&self) -> String;
}

trait ParameterAST: ExprAST { }

struct LiteralExprAST<T> {
    pub value: T,
}

impl<T: ToString> ExprAST for LiteralExprAST<T> {
    fn to_rust(&self) -> String {
        self.value.to_string()
    }
}

impl<T: ToString> ParameterAST for LiteralExprAST<T> { }

struct VariableExprAST {
    pub name: String,
}

impl ExprAST for VariableExprAST {
    fn to_rust(&self) -> String {
        self.name.clone()
    }
}

impl ParameterAST for VariableExprAST { }

struct BinaryExprAST<'ast> {
    pub op: String,
    pub lhs: &'ast dyn ExprAST,
    pub rhs: &'ast dyn ExprAST,
}

impl ExprAST for BinaryExprAST<'_> {
    fn to_rust(&self) -> String {
        format!("{} {} {}", self.lhs.to_rust(), self.op, self.rhs.to_rust())
    }
}

struct CallExprAST<'ast> {
    pub callee: String,
    pub args: Vec<&'ast dyn ExprAST>,
}

impl ExprAST for CallExprAST<'_> {
    fn to_rust(&self) -> String {
        format!(
            "{}({})",
            self.callee,
            self.args.iter()
                .map(|arg| arg.to_rust())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

struct AssignExprAST<'ast> {
    pub name: String,
    pub mutable: bool,
    pub value: &'ast dyn ExprAST,
}

impl ExprAST for AssignExprAST<'_> {
    fn to_rust(&self) -> String {
        let mutability = if self.mutable { "mut " } else { "" };
        format!("let {}{} = {};", mutability, self.name, self.value.to_rust())
    }
}

struct TypeSignatureAST {
    pub name: String,
    pub arg_types: Vec<String>,
    pub return_type: String,
}

struct EquationAST<'ast> {
    pub parameters_list: Vec<&'ast dyn ParameterAST>,
    pub body: Vec<&'ast dyn ExprAST>,
}

struct FunctionAST<'ast> {
    pub signature: TypeSignatureAST,
    pub equations: Vec<EquationAST<'ast>>,
}

impl FunctionAST<'_> {
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
        let head = format!("fn {}({}) -> {}", signature.name, param, signature.return_type);
        let body = equation.body.iter()
            .map(|expr| expr.to_rust())
            .collect::<Vec<_>>()
            .join("\n    ");
        format!("{} {{\n    {}\n}}", head, body)
    }
}

struct ProgramAST<'ast> {
    pub functions: Vec<FunctionAST<'ast>>,
    pub variables: Vec<AssignExprAST<'ast>>,
}

impl ProgramAST<'_> {
    pub fn to_rust(&self) -> String {
        let functions = self.functions.iter()
            .map(|f| f.to_rust())
            .collect::<Vec<_>>()
            .join("\n\n");
        let variables = self.variables.iter()
            .map(|v| v.to_rust())
            .collect::<Vec<_>>()
            .join("\n");
        format!("{}\n\n{}", variables, functions)
    }
}