From Stdlib Require Export String.
From Stdlib Require Export ZArith.
Open Scope string_scope.

(* leave out R, Ref, Option, Result, Generic, and Array types *)
Inductive soulite_type: Type :=
| TypeN (bits: nat)
| TypeZ (bits: nat)
| TypeString
| TypeList (type: soulite_type)
| TypeClosure (param_types return_types: list soulite_type)
.

Inductive binop: Type :=
| Add           (* +  *)
| Sub           (* -  *)
| Mul           (* *  *)
| Div           (* /  *)
| LessThan      (* <  *)
| Lte           (* <= *)
| GreaterThan   (* >  *)
| Gte           (* >= *)
| Equal         (* == *)
| And           (* && *)
| Or            (* || *)
| ShiftLeft     (* << *)
| EndLeft       (* <| *)
.

(* leave out This, None, Ref, Some, Ok, and Err expressions *)
Inductive soulite_expr: Type :=
| Skip
| N (n: nat) (* assume in range of bits *)
| Z (n: Z)   (* assume in range of bits *)
| String (val: string)
| Var (name: string)
| List (exprs: list soulite_expr)
| Binary (op: binop) (lhs rhs: soulite_expr)
| Ternary (cond if_true if_false: soulite_expr)
| Call (name: string) (args: list soulite_expr)
(* assume that type inferrence is not used and type is always provided *)
| Declare (name: string) (mutable: bool) (expr: soulite_expr) (type: soulite_type)
| Assign (name: string) (expr: soulite_expr)
| Closure (args: list string) (body: soulite_expr)
| While (cond body: soulite_expr)
| Return (expr: soulite_expr)
| Seq (e1 e2: soulite_expr)
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