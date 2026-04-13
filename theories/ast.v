From Stdlib Require Export String.
From Stdlib Require Export ZArith.
Open Scope string_scope.
Open Scope Z_scope.

(* leave out R, Ref, Option, Result, Generic, and Array types *)
Inductive sl_type: Type :=
| TypeN
| TypeZ
| TypeString
| TypeList (type: sl_type)
| TypeClosure (param_types return_type: list sl_type)
.

Inductive binop: Type :=
| PlusOp        (* +  *)
| MinusOp       (* -  *)
| MultOp        (* *  *)
| DivOp         (* /  *)
| LtOp          (* <  *)
| LteOp         (* <= *)
| GtOp          (* >  *)
| GteOp         (* >= *)
| EqOp          (* == *)
| NotEqOp       (* != *)
| AndOp         (* && *)
| OrOp          (* || *)
| ShiftLeftOp   (* << *)
| EndLeftOp     (* <| *)
| DotOp         (* .  *)
.

(* leave out This, None, Ref, Some, Ok, and Err expressions *)
Inductive sl_expr: Type :=
| Skip
| NExpr (n: nat) (* assume in range of bits *)
| ZExpr (n: Z)   (* assume in range of bits *)
| StringExpr (val: string)
| VarExpr (name: string)
| ListExpr (exprs: list sl_expr)
| BinaryExpr (op: binop) (lhs rhs: sl_expr)
| TernaryExpr (cond if_true if_false: sl_expr)
| CallExpr (name: string) (args: list sl_expr)
(* assume that type inferrence is not used and type is always provided *)
| DeclareExpr (name: string) (mutable: bool) (type: sl_type) (expr: sl_expr)
| AssignExpr (name: string) (expr: sl_expr)
| ClosureExpr (args: list string) (body: sl_expr)
| WhileExpr (cond body: sl_expr)
| ReturnExpr (expr: sl_expr)
| SeqExpr (e1 e2: sl_expr)
.

Record sl_function := {
    name: string;
    params: list string;
    param_types: list sl_type;
    return_type: list sl_type;
    body: sl_expr;
}.

Definition program := prod (list sl_function) sl_expr.

Definition empty_program : program := (nil, Skip).