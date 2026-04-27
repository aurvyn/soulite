From Stdlib Require Import String ZArith List.

(* leave out R, Ref, Option, Result, Generic, and Array types *)
Inductive sl_type :=
| TypeBool
| TypeZ
| TypeString
| TypeList (type: sl_type)
| TypeClosure (param_types return_type: list sl_type)
.

Inductive unop :=
| NegOp     (* -  *)
| NotOp     (* ~  *)
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
.

Inductive sl_lit :=
| LitBoolean (b: bool)
| LitZ (n: Z)
| LitString (val: string)
| LitList (lits: list sl_lit)
.

(* leave out This, None, Ref, Some, Ok, and Err expressions *)
Inductive sl_expr :=
| ValExpr (val: sl_val)
| VarExpr (name: string)
| ListExpr (exprs: list sl_expr)
| UnaryExpr (op: unop) (expr: sl_expr)
| BinaryExpr (op: binop) (lhs rhs: sl_expr)
| TernaryExpr (cond if_true if_false: sl_expr)
| CallClosureExpr (closure: sl_expr) (args: list sl_expr)
(* | CallFunctionExpr (func: string) (args: list sl_expr) *)
(* assume that type inferrence is not used and type is always provided *)
| DeclareExpr (name: string) (mutable: bool) (type: sl_type) (expr: sl_expr)
| AssignExpr (name: string) (expr: sl_expr)
| ClosureExpr (params: list string) (body: sl_expr)
| WhileExpr (cond body: sl_expr)
| SeqExpr (e1 e2: sl_expr)
with sl_val :=
| LitVal (lit: sl_lit)
| ClosureVal (params: list string) (body: sl_expr)
.

Definition alloc_length (e: sl_expr): nat :=
    match e with
    | ListExpr exprs => length exprs
    | ValExpr (LitVal (LitList lits)) => length lits
    | _ => 1
    end.

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
    | UnaryExpr op expr => UnaryExpr op (subst x v expr)
    | BinaryExpr op lhs rhs => BinaryExpr op (subst x v lhs) (subst x v rhs)
    | TernaryExpr cond if_true if_false => TernaryExpr (subst x v cond) (subst x v if_true) (subst x v if_false)
    | CallClosureExpr closure args => CallClosureExpr (subst x v closure) (map (subst x v) args)
 (* | CallFunctionExpr func args => CallFunctionExpr func (map (subst x v) args) *)
    | DeclareExpr name mutable type expr => DeclareExpr name mutable type (subst x v expr)
    | AssignExpr name expr => AssignExpr name (subst x v expr)
    | ClosureExpr params body => if existsb (eqb x) params then e else ClosureExpr params (subst x v body)
    | WhileExpr cond body => WhileExpr (subst x v cond) (subst x v body)
    | SeqExpr e1 e2 => SeqExpr (subst x v e1) (subst x v e2)
    end.

Record sl_func := {
    name: string;
    params: list string;
    param_types: list sl_type;
    return_types: list sl_type;
    body: sl_expr;
}.

Definition program := prod (list sl_func) sl_expr.