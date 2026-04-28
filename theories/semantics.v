From Stdlib Require Import String.
From stdpp Require Import gmap.
From Soulite Require Import ast notation.

Fixpoint list_eqb {A: Type} (eqb_A: A -> A -> bool) (l1 l2: list A): bool :=
    match l1, l2 with
    | [], [] => true
    | h1 :: t1, h2 :: t2 => eqb_A h1 h2 && list_eqb eqb_A t1 t2
    | _, _ => false
    end.

Fixpoint sl_lit_eqb (lit1 lit2: sl_lit): bool :=
    match lit1, lit2 with
    | LitBoolean b1, LitBoolean b2 => Bool.eqb b1 b2
    | LitZ n1, LitZ n2 => Z.eqb n1 n2
    | LitString s1, LitString s2 => String.eqb s1 s2
    | LitList l1, LitList l2 => list_eqb sl_lit_eqb l1 l2
    | _, _ => false
    end.

Definition binop_eval_bool (op: binop) (b1 b2: bool): option sl_lit :=
    LitBoolean <$> match op with
    | EqOp => Some (eqb b1 b2)
    | NotEqOp => Some (negb (eqb b1 b2))
    | AndOp => Some (b1 && b2)
    | OrOp => Some (b1 || b2)
    | _ => None
    end.

Definition binop_eval_Z (op: binop) (n1 n2: Z): option sl_lit :=
    match op with
    | PlusOp => Some (LitZ (n1 + n2))
    | MinusOp => Some (LitZ (n1 - n2))
    | MultOp => Some (LitZ (n1 * n2))
    | DivOp => Some (LitZ (Z.quot n1 n2))
    | ModOp => Some (LitZ (Z.rem n1 n2))
    | LtOp => Some (LitBoolean (Z.ltb n1 n2))
    | LeOp => Some (LitBoolean (Z.leb n1 n2))
    | GtOp => Some (LitBoolean (Z.ltb n2 n1))
    | GeOp => Some (LitBoolean (Z.leb n2 n1))
    | EqOp => Some (LitBoolean (Z.eqb n1 n2))
    | NotEqOp => Some (LitBoolean (negb (Z.eqb n1 n2)))
    | AndOp => Some (LitZ (Z.land n1 n2))
    | OrOp => Some (LitZ (Z.lor n1 n2))
    | ShiftLOp => Some (LitZ (Z.shiftl n1 n2))
    | ShiftROp => Some (LitZ (Z.shiftr n1 n2))
    end.

Definition binop_eval_string (op: binop) (s1 s2: string): option sl_lit :=
    match op with
    | PlusOp => Some (LitString (s1 ++ s2))
    | EqOp => Some (LitBoolean (String.eqb s1 s2))
    | NotEqOp => Some (LitBoolean (negb (String.eqb s1 s2)))
    | _ => None
    end.

Definition binop_eval_list (op: binop) (l1 l2: list sl_lit): option sl_lit :=
    match op with
    | PlusOp => Some (LitList (l1 ++ l2))
    | EqOp => Some (LitBoolean (list_eqb sl_lit_eqb l1 l2))
    | NotEqOp => Some (LitBoolean (negb (list_eqb sl_lit_eqb l1 l2)))
    | _ => None
    end.

Definition binop_eval (op: binop) (v1 v2: sl_val): option sl_val :=
    LitVal <$> match v1, v2 with
    | LitVal (LitBoolean b1), LitVal (LitBoolean b2) => binop_eval_bool op b1 b2
    | LitVal (LitZ n1), LitVal (LitZ n2) => binop_eval_Z op n1 n2
    | LitVal (LitString s1), LitVal (LitString s2) => binop_eval_string op s1 s2
    | LitVal (LitList l1), LitVal (LitList l2) => binop_eval_list op l1 l2
    | _, _ => None
    end.

Fixpoint subst_list (strs: list string) (vals: list sl_val) (expr: sl_expr): sl_expr :=
    match strs, vals with
    | str :: strs', val :: vals' => subst_list strs' vals' (subst str val expr)
    | _, _ => expr
    end.

(* unchanged state shorthand *)
Reserved Notation "a =-> b ; state" (at level 90, format "a  =->  b ;  state").

Inductive sl_step : sl_expr * sl_state -> sl_expr * sl_state -> Prop :=
| VarStep var state val
    (Hexist: state.(env) !! var = Some val):
        VarExpr var =-> ValExpr val; state
| AssignStep var expr val val' state
    (Hval: to_val expr = Some val')
    (Hexist: state.(env) !! var = Some val):
        sl_step (<{var ,= expr}>, state)
                (ValExpr val', Build_sl_state (<[var := val']> state.(env)) state.(heap))
| DeclareImmutStep var type expr val state
    (Hval: to_val expr = Some val):
        sl_step (<{var: type = expr}>, state)
                (ValExpr val, Build_sl_state (<[var := val]> state.(env)) state.(heap))
| DeclareMutStep var type expr val state
    (Hval: to_val expr = Some val):
        sl_step (<{var; type = expr}>, state)
                (ValExpr val, Build_sl_state (<[var := val]> state.(env)) state.(heap))
| BinOpStepL op e1 e2 e1' state state'
    (Hstep: sl_step (e1, state) (e1', state')):
        sl_step (BinaryExpr op e1 e2, state) (BinaryExpr op e1' e2, state')
| BinOpStepR op e1 e2 e2' val state state'
    (Hval: to_val e1 = Some val)
    (Hstep: sl_step (e2, state) (e2', state')):
        sl_step (BinaryExpr op (ValExpr val) e2, state) (BinaryExpr op (ValExpr val) e2', state')
| BinOpStep op e1 e2 v1 v2 val state
    (Hval1: to_val e1 = Some v1)
    (Hval2: to_val e2 = Some v2)
    (Hbinop: binop_eval op v1 v2 = Some val):
        BinaryExpr op e1 e2 =-> ValExpr val; state
| SeqConsStep e1 e1' e2 state state'
    (Hstep: sl_step (e1, state) (e1', state')):
        sl_step (SeqExpr e1 e2, state) (SeqExpr e1' e2, state')
| SeqValStep val expr state:
        SeqExpr (ValExpr val) expr =-> expr; state
| WhileTrue cond body state
    (Htrue: to_val cond = Some (LitVal (LitBoolean true))):
        WhileExpr cond body =-> SeqExpr body (WhileExpr cond body); state
| WhileFalse cond body state
    (Hfalse: to_val cond = Some (LitVal (LitBoolean false))):
        WhileExpr cond body =-> ValExpr (LitVal (LitZ 0)); state
| CallClosureStep func args argvs param_names body state
    (Hfunc: to_val func = Some (ClosureVal param_names body))
    (Hargs: Forall2 (λ e v, to_val e = Some v) args argvs)
    (Hlen: length param_names = length argvs):
        CallClosureExpr func args =-> subst_list param_names argvs body; state
where "a =-> b ; state" := (sl_step (a, state) (b, state)) : sl_scope.