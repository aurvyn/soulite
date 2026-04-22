From Stdlib Require Import String.
From Stdlib Require Import ZArith.
From stdpp Require Import gmap.
From Soulite Require Import ast.
From Soulite Require Import notation.

Definition binop_eval_Z (op: binop) (n1 n2: Z): option sl_lit :=
    match op with
    | PlusOp => Some (LitZ (n1 + n2))
    | MinusOp => Some (LitZ (n1 - n2))
    | MultOp => Some (LitZ (n1 * n2))
    | QuotOp => Some (LitZ (Z.quot n1 n2))
    | RemOp => Some (LitZ (Z.rem n1 n2))
    | LtOp => Some (LitBool (bool_decide (n1 < n2)))
    | LeOp => Some (LitBool (bool_decide (n1 <= n2)))
    | GtOp => Some (LitBool (bool_decide (n1 > n2)))
    | GeOp => Some (LitBool (bool_decide (n1 >= n2)))
    | EqOp => Some (LitBool (bool_decide (n1 = n2)))
    | NotEqOp => Some (LitBool (bool_decide (n1 <> n2)))
    | AndOp => Some (LitZ (Z.land n1 n2))
    | OrOp => Some (LitZ (Z.lor n1 n2))
    | ShiftLOp => Some (LitZ (Z.shiftl n1 n2))
    | ShiftROp => Some (LitZ (Z.shiftr n1 n2))
    | _ => None
    end%Z.

Definition binop_eval (op: binop) (v1 v2: sl_val): option sl_val :=
  match v1, v2 with
  | LitVal (LitZ n1), LitVal (LitZ n2) => LitVal <$> binop_eval_Z op n1 n2
  | _, _ => None
  end.

Record sl_state := {
  env: gmap string sl_val;
  heap: gmap Z sl_val; (* not used yet, maybe for allocating lists later? *)
}.

Fixpoint subst_list (xs: list string) (vs: list sl_val) (e: sl_expr): sl_expr :=
    match xs, vs with
    | x :: xs', v :: vs' => subst_list xs' vs' (subst x v e)
    | _, _ => e
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
    sl_step (Seq (expr :: exprs), state) (Seq (expr' :: exprs), state')
| SeqValStep val exprs state:
    sl_step (Seq (ValExpr val :: exprs), state) (Seq exprs, state)
| SeqNilStep state:
    sl_step (Seq [], state) (ValExpr (LitVal (LitZ 0)), state)
| WhileTrue cond body state:
    to_val cond = Some (LitVal (LitBool true)) ->
    sl_step (WhileExpr cond body, state) (Seq [body; WhileExpr cond body], state)
| WhileFalse cond body state:
    to_val cond = Some (LitVal (LitBool false)) ->
    sl_step (WhileExpr cond body, state) (ValExpr (LitVal (LitZ 0)), state)
| CallClosureStep func args v_args param_names body state:
    to_val func = Some (ClosureVal param_names body) ->
    Forall2 (λ e v, to_val e = Some v) args v_args ->
    length param_names = length v_args ->
    let exprs := subst_list param_names v_args body in
    sl_step (CallExpr func args, state) (exprs, state).