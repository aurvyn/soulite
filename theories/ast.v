From Stdlib Require Export String.
From Stdlib Require Export ZArith.
Open Scope string_scope.
Open Scope Z_scope.

(* leave out R, Ref, Option, Result, Generic, and Array types *)
Inductive sl_type :=
| TypeN
| TypeZ
| TypeString
| TypeList (type: sl_type)
| TypeClosure (param_types return_type: list sl_type)
.

Inductive binop :=
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
Inductive sl_expr :=
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
| SeqExpr (e1 e2: sl_expr)
.

Inductive sl_val :=
| NVal (n: nat)
| ZVal (n: Z)
| StringVal (val: string)
| ListVal (vals: list sl_val)
| ClosureVal (args: list string) (body: sl_expr)
.

Fixpoint of_val (v: sl_val): sl_expr :=
    match v with
    | NVal n => NExpr n
    | ZVal n => ZExpr n
    | StringVal val => StringExpr val
    | ListVal vals => ListExpr (List.map of_val vals)
    | ClosureVal args body => ClosureExpr args body
    end.

Fixpoint to_val (e: sl_expr): option sl_val :=
    match e with
    | NExpr n => Some (NVal n)
    | ZExpr n => Some (ZVal n)
    | StringExpr val => Some (StringVal val)
    | ListExpr exprs =>
        let fix test (tail: list sl_expr): option (list sl_val) :=
            match tail with
            | nil => Some nil
            | cons x xs => match to_val x with
                | None => None
                | Some val => match test xs with
                    | None => None
                    | Some vals => Some (cons val vals)
        end end end in
        match test exprs with
        | None => None
        | Some vals => Some (ListVal vals)
        end
    | ClosureExpr args body => Some (ClosureVal args body)
    | _ => None
    end.

Definition is_val (e: sl_expr): Prop :=
    match to_val e with
    | Some _ => True
    | None => False
    end.

Record sl_function := {
    name: string;
    params: list string;
    param_types: list sl_type;
    return_type: list sl_type;
    body: sl_expr;
}.

Definition program := prod (list sl_function) sl_expr.

Definition empty_program : program := (nil, Skip).