From Stdlib Require Import String.
From Stdlib Require Import NArith.
From Stdlib Require Import ZArith.
From Stdlib Require Import List.

(* leave out R, Ref, Option, Result, Generic, and Array types *)
Inductive sl_type :=
| TypeN
| TypeZ
| TypeString
| TypeList (type: sl_type)
| TypeClosure (param_types return_type: list sl_type)
.

Inductive binop :=
| PlusOp    (* +  *)
| MinusOp   (* -  *)
| MultOp    (* *  *)
| DivOp     (* /  *)
| ModOp     (* %  *)
| LtOp      (* <  *)
| LeOp      (* <= *)
| GtOp      (* >  *)
| GeOp      (* >= *)
| EqOp      (* == *)
| NotEqOp   (* != *)
| AndOp     (* && *)
| OrOp      (* || *)
| ShiftLOp  (* << *)
| ShiftROp  (* >> *)
| EndLOp    (* <| *)
| DotOp     (* .  *)
.

Inductive sl_lit :=
| LitBool (b: bool)
| LitN (n: N)
| LitZ (n: Z)
| LitString (val: string)
| LitList (vals: list sl_lit)
.

(* leave out This, None, Ref, Some, Ok, and Err expressions *)
Inductive sl_expr :=
| ValExpr (v: sl_val)
| VarExpr (name: string)
| ListExpr (exprs: list sl_expr)
| BinaryExpr (op: binop) (lhs rhs: sl_expr)
| TernaryExpr (cond if_true if_false: sl_expr)
| CallExpr (func: sl_expr) (args: list sl_expr)
(* assume that type inferrence is not used and type is always provided *)
| DeclareExpr (name: string) (mutable: bool) (type: sl_type) (expr: sl_expr)
| AssignExpr (name: string) (expr: sl_expr)
| ClosureExpr (args: list string) (body: sl_expr)
| WhileExpr (cond body: sl_expr)
| Seq (exprs: list sl_expr)
with sl_val :=
| LitVal (lit: sl_lit)
| ClosureVal (args: list string) (body: sl_expr)
.

Notation of_val := ValExpr (only parsing).

Definition to_val (e: sl_expr): option sl_val :=
    match e with
    | of_val v => Some v
    | _ => None
    end.

Lemma to_of_val v: to_val (of_val v) = Some v.
Proof.
    destruct v; reflexivity.
Qed.

Lemma of_to_val e v: to_val e = Some v -> of_val v = e.
Proof.
    destruct e; intro H; try discriminate.
    f_equal. injection H as H. symmetry. assumption.
Qed.

Fixpoint subst (x: string) (v: sl_val) (e: sl_expr): sl_expr :=
    match e with
    | ValExpr _ => e
    | VarExpr name => if eqb name x then of_val v else VarExpr name
    | ListExpr exprs => ListExpr (map (subst x v) exprs)
    | BinaryExpr op lhs rhs => BinaryExpr op (subst x v lhs) (subst x v rhs)
    | TernaryExpr cond if_true if_false => TernaryExpr (subst x v cond) (subst x v if_true) (subst x v if_false)
    | CallExpr func args => CallExpr (subst x v func) (map (subst x v) args)
    | DeclareExpr name mutable type expr => DeclareExpr name mutable type (subst x v expr)
    | AssignExpr name expr => AssignExpr name (subst x v expr)
    | ClosureExpr args body => if existsb (eqb x) args then e else ClosureExpr args (subst x v body)
    | WhileExpr cond body => WhileExpr (subst x v cond) (subst x v body)
    | Seq exprs => Seq (map (subst x v) exprs)
    end.

Record sl_function := {
    name: string;
    params: list string;
    param_types: list sl_type;
    return_type: list sl_type;
    body: sl_expr;
}.

Definition program := prod (list sl_function) sl_expr.