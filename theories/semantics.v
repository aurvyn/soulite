From Stdlib Require Import String.
From Stdlib Require Import NArith.
From Stdlib Require Import ZArith.
From Stdlib Require Import List.
From stdpp Require Import gmap.
From Soulite Require Import ast.
From Soulite Require Import notation.
Import ListNotations.

Fixpoint list_eqb {A: Type} (eqb_A: A -> A -> bool) (l1 l2: list A): bool :=
    match l1, l2 with
    | [], [] => true
    | h1 :: t1, h2 :: t2 => eqb_A h1 h2 && list_eqb eqb_A t1 t2
    | _, _ => false
    end.

Fixpoint sl_lit_eqb (lit1 lit2: sl_lit): bool :=
    match lit1, lit2 with
    | LitBool b1, LitBool b2 => Bool.eqb b1 b2
    | LitN n1, LitN n2 => N.eqb n1 n2
    | LitZ z1, LitZ z2 => Z.eqb z1 z2
    | LitString s1, LitString s2 => String.eqb s1 s2
    | LitList l1, LitList l2 => list_eqb sl_lit_eqb l1 l2
    | _, _ => false
    end.

Definition binop_eval_bool (op: binop) (b1 b2: bool): option sl_lit :=
    LitBool <$> match op with
    | EqOp => Some (eqb b1 b2)
    | NotEqOp => Some (negb (eqb b1 b2))
    | AndOp => Some (b1 && b2)
    | OrOp => Some (b1 || b2)
    | _ => None
    end.

Definition binop_eval_N (op: binop) (n1 n2: N): option sl_lit :=
    match op with
    | PlusOp => Some (LitN (n1 + n2))
    | MinusOp => Some (LitN (n1 - n2))
    | MultOp => Some (LitN (n1 * n2))
    | DivOp => Some (LitN (N.div n1 n2))
    | ModOp => Some (LitN (N.modulo n1 n2))
    | LtOp => Some (LitBool (N.ltb n1 n2))
    | LeOp => Some (LitBool (N.leb n1 n2))
    | GtOp => Some (LitBool (N.ltb n2 n1))
    | GeOp => Some (LitBool (N.leb n2 n1))
    | EqOp => Some (LitBool (N.eqb n1 n2))
    | NotEqOp => Some (LitBool (negb (N.eqb n1 n2)))
    | AndOp => Some (LitN (N.land n1 n2))
    | OrOp => Some (LitN (N.lor n1 n2))
    | ShiftLOp => Some (LitN (N.shiftl n1 n2))
    | ShiftROp => Some (LitN (N.shiftr n1 n2))
    | _ => None
    end.

Definition binop_eval_Z (op: binop) (z1 z2: Z): option sl_lit :=
    match op with
    | PlusOp => Some (LitZ (z1 + z2))
    | MinusOp => Some (LitZ (z1 - z2))
    | MultOp => Some (LitZ (z1 * z2))
    | DivOp => Some (LitZ (Z.quot z1 z2))
    | ModOp => Some (LitZ (Z.rem z1 z2))
    | LtOp => Some (LitBool (Z.ltb z1 z2))
    | LeOp => Some (LitBool (Z.leb z1 z2))
    | GtOp => Some (LitBool (Z.ltb z2 z1))
    | GeOp => Some (LitBool (Z.leb z2 z1))
    | EqOp => Some (LitBool (Z.eqb z1 z2))
    | NotEqOp => Some (LitBool (negb (Z.eqb z1 z2)))
    | AndOp => Some (LitZ (Z.land z1 z2))
    | OrOp => Some (LitZ (Z.lor z1 z2))
    | ShiftLOp => Some (LitZ (Z.shiftl z1 z2))
    | ShiftROp => Some (LitZ (Z.shiftr z1 z2))
    | _ => None
    end.

Definition binop_eval_string (op: binop) (s1 s2: string): option sl_lit :=
    match op with
    | PlusOp => Some (LitString (s1 ++ s2))
    | EqOp => Some (LitBool (String.eqb s1 s2))
    | NotEqOp => Some (LitBool (negb (String.eqb s1 s2)))
    | _ => None
    end.

Definition binop_eval_list (op: binop) (l1 l2: list sl_lit): option sl_lit :=
    match op with
    | PlusOp => Some (LitList (l1 ++ l2))
    | EqOp => Some (LitBool (list_eqb sl_lit_eqb l1 l2))
    | NotEqOp => Some (LitBool (negb (list_eqb sl_lit_eqb l1 l2)))
    | _ => None
    end.

Definition binop_eval (op: binop) (v1 v2: sl_val): option sl_val :=
    LitVal <$> match v1, v2 with
    | LitVal (LitBool b1), LitVal (LitBool b2) => binop_eval_bool op b1 b2
    | LitVal (LitN n1), LitVal (LitN n2) => binop_eval_N op n1 n2
    | LitVal (LitZ n1), LitVal (LitZ n2) => binop_eval_Z op n1 n2
    | LitVal (LitString s1), LitVal (LitString s2) => binop_eval_string op s1 s2
    | LitVal (LitList l1), LitVal (LitList l2) => binop_eval_list op l1 l2
    | _, _ => None
    end.

Record sl_state := {
    env: gmap string sl_val;
    heap: gmap Z sl_val; (* not used yet, maybe for allocating lists later? *)
}.

Fixpoint subst_list (strs: list string) (vals: list sl_val) (expr: sl_expr): sl_expr :=
    match strs, vals with
    | str :: strs', val :: vals' => subst_list strs' vals' (subst str val expr)
    | _, _ => expr
    end.

Inductive sl_step : sl_expr * sl_state -> sl_expr * sl_state -> Prop :=
| VarStep var state val:
    state.(env) !! var = Some val ->
    sl_step (VarExpr var, state) (ValExpr val, state)
| AssignStep var expr val val' state:
    to_val expr = Some val' ->
    state.(env) !! var = Some val ->
    sl_step (<{var ,= expr}>, state)
            (ValExpr val',
                {| env := <[var := val']> state.(env); heap := state.(heap) |})
| DeclareImmutStep var type expr val state:
    to_val expr = Some val ->
    sl_step (<{var: type = expr}>, state)
            (ValExpr val,
                {| env := <[var := val]> state.(env); heap := state.(heap) |})
| DeclareMutStep var type expr val state:
    to_val expr = Some val ->
    sl_step (<{var; type = expr}>, state)
            (ValExpr val,
                {| env := <[var := val]> state.(env); heap := state.(heap) |})
| BinOpStep op e1 e2 v1 v2 val state:
    to_val e1 = Some v1 ->
    to_val e2 = Some v2 ->
    binop_eval op v1 v2 = Some val ->
    sl_step (BinaryExpr op e1 e2, state) (ValExpr val, state)
| SeqConsStep expr expr' exprs state state':
    sl_step (expr, state) (expr', state') ->
    sl_step (SeqExpr (expr :: exprs), state) (SeqExpr (expr' :: exprs), state')
| SeqValStep val exprs state:
    sl_step (SeqExpr (ValExpr val :: exprs), state) (SeqExpr exprs, state)
| SeqNilStep state:
    sl_step (SeqExpr [], state) (ValExpr (LitVal (LitZ 0)), state)
| WhileTrue cond body state:
    to_val cond = Some (LitVal (LitBool true)) ->
    sl_step (WhileExpr cond body, state) (SeqExpr [body; WhileExpr cond body], state)
| WhileFalse cond body state:
    to_val cond = Some (LitVal (LitBool false)) ->
    sl_step (WhileExpr cond body, state) (ValExpr (LitVal (LitZ 0)), state)
| CallClosureStep func args v_args param_names body state:
    to_val func = Some (ClosureVal param_names body) ->
    Forall2 (λ e v, to_val e = Some v) args v_args ->
    length param_names = length v_args ->
    let exprs := subst_list param_names v_args body in
    sl_step (CallExpr func args, state) (exprs, state).