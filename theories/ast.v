From Stdlib Require Export String.
From Stdlib Require Export ZArith.
Open Scope string_scope.
Open Scope Z_scope.

(* leave out R, Ref, Option, Result, Generic, and Array types *)
Inductive soulite_type: Type :=
| TypeN (bits: nat)
| TypeZ (bits: nat)
| TypeString
| TypeList (type: soulite_type)
| TypeClosure (param_types return_types: list soulite_type)
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
Inductive soulite_expr: Type :=
| Skip
| NExpr (n: nat) (* assume in range of bits *)
| ZExpr (n: Z)   (* assume in range of bits *)
| StringExpr (val: string)
| VarExpr (name: string)
| ListExpr (exprs: list soulite_expr)
| BinaryExpr (op: binop) (lhs rhs: soulite_expr)
| TernaryExpr (cond if_true if_false: soulite_expr)
| CallExpr (name: string) (args: list soulite_expr)
(* assume that type inferrence is not used and type is always provided *)
| DeclareExpr (name: string) (mutable: bool) (type: soulite_type) (expr: soulite_expr)
| AssignExpr (name: string) (expr: soulite_expr)
| ClosureExpr (args: list string) (body: soulite_expr)
| WhileExpr (cond body: soulite_expr)
| ReturnExpr (expr: soulite_expr)
| SeqExpr (e1 e2: soulite_expr)
.

Record function := {
    name: string;
    params: list string;
    param_types: list soulite_type;
    body: soulite_expr;
    return_types: list soulite_type;
}.

Definition program := prod (list function) soulite_expr.

Definition empty_program : program := (nil, Skip).