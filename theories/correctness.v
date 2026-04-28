From Stdlib Require Import String ZArith List Lia.
From Stdlib.Relations Require Import Relation_Operators.
From stdpp Require Import gmap.
From iris.heap_lang Require Import lang proofmode notation.
From iris.heap_lang.lib Require Import array.
From iris.program_logic Require Import weakestpre.
From Soulite Require Import ast notation semantics heaplang.

Import heap_lang.

Section correctness.
Context `{!heapGS Σ}.

Definition sl_terminates (e: sl_expr) (v: sl_val): Prop :=
    exists σ', rtc sl_step (e, empty_state) (ValExpr v, σ').

Inductive closed_expr (bound: list string): sl_expr -> Prop :=
| ClosedVal v:
        closed_expr bound (ValExpr v)
| ClosedVar name
    (Hbound: List.In name bound):
        closed_expr bound <{!name}>
| ClosedList exprs
    (Hclosed: Forall (closed_expr bound) exprs):
        closed_expr bound (ListExpr exprs)
| ClosedUnary op expr
    (Hclosed: closed_expr bound expr) :
        closed_expr bound (UnaryExpr op expr)
| ClosedBinary op lhs rhs
    (Hlhs: closed_expr bound lhs)
    (Hrhs: closed_expr bound rhs):
        closed_expr bound (BinaryExpr op lhs rhs)
| ClosedTernary cond if_true if_false
    (Hcond: closed_expr bound cond)
    (Htrue: closed_expr bound if_true)
    (Hfalse: closed_expr bound if_false):
        closed_expr bound <{if_true <- cond;; if_false}>
| ClosedCallClosure closure args
    (Hclosure: closed_expr bound closure)
    (Hargs: Forall (closed_expr bound) args):
        closed_expr bound (CallClosureExpr closure args)
| ClosedDeclare name mut type expr
    (Hclosed: closed_expr (name :: bound) expr):
        closed_expr bound (DeclareExpr name mut type expr)
| ClosedAssign name expr
    (Hbound: List.In name bound)
    (Hclosed: closed_expr bound expr):
        closed_expr bound <{name ,= expr}>
| ClosedClosure params body
    (Hclosed: closed_expr (params ++ bound) body):
        closed_expr bound (ClosureExpr params body)
| ClosedWhile cond body
    (Hcond: closed_expr bound cond)
    (Hbody: closed_expr bound body):
        closed_expr bound (WhileExpr cond body)
| ClosedSeqDecl name mut type expr e2
    (Hexpr: closed_expr bound expr)
    (Hbody: closed_expr (name :: bound) e2):
        closed_expr bound (SeqExpr (DeclareExpr name mut type expr) e2)
| ClosedSeq e1 e2
    (Hleft: closed_expr bound e1)
    (Hright: closed_expr bound e2):
        closed_expr bound <{e1 \n\t e2}>.

Definition closed_program (e: sl_expr): Prop :=
    closed_expr [] e.

(*
The current compiler treats mutable declarations as references and assignments as stores.
SimpleSoulite semantics does not model reference values yet, so for now we restrict to the
immutable fragment to keep the correspondence statement simple.
*)
Inductive pure_expr: sl_expr -> Prop :=
| PureVal v:
        pure_expr (ValExpr v)
| PureVar name:
        pure_expr <{!name}>
| PureList exprs
    (Hpure: Forall pure_expr exprs):
        pure_expr (ListExpr exprs)
| PureUnary op expr
    (Hpure: pure_expr expr):
        pure_expr (UnaryExpr op expr)
| PureBinary op lhs rhs
    (Hlhs: pure_expr lhs)
    (Hrhs: pure_expr rhs):
        pure_expr (BinaryExpr op lhs rhs)
| PureTernary cond if_true if_false
    (Hcond: pure_expr cond)
    (Htrue: pure_expr if_true)
    (Hfalse: pure_expr if_false):
        pure_expr <{if_true <- cond;; if_false}>
| PureCallClosure closure args
    (Hclosure: pure_expr closure)
    (Hargs: Forall pure_expr args):
        pure_expr (CallClosureExpr closure args)
| PureDeclare name type expr
    (Hpure: pure_expr expr):
        pure_expr (DeclareExpr name false type expr)
| PureClosure params body (Hpure: pure_expr body):
        pure_expr (ClosureExpr params body)
| PureWhile cond body
    (Hcond: pure_expr cond)
    (Hbody: pure_expr body):
        pure_expr (WhileExpr cond body)
| PureSeq e1 e2
    (Hleft: pure_expr e1)
    (Hright: pure_expr e2):
        pure_expr <{e1 \n\t e2}>.

Fixpoint string_array_rel (l: loc) (s: string): iProp Σ :=
    match s with
    | "" => True%I
    | String c s' =>
        l ↦ #c ∗ string_array_rel (l +ₗ 1%Z) s'
    end.

Lemma compile_str_correct (l: loc) (s: string):
    l ↦∗ replicate (String.length s) #0 -∗
    WP (compile_str #l s) {{ _, string_array_rel l s }}.
Admitted.

Fixpoint lit_rel (lit: sl_lit) (v: val) {struct lit}: iProp Σ :=
    match lit with
    | LitBoolean b => ⌜v = #b⌝
    | LitZ n => ⌜v = #n⌝
    | LitString s =>
        if Nat.eqb (String.length s) 0 then ⌜v = NONEV⌝
        else ∃ l : loc, ⌜v = #l⌝ ∗ string_array_rel l s
    | LitList lits =>
        let fix list_rel (lits: list sl_lit) (v: val): iProp Σ :=
            match lits with
            | [] => ⌜v = NONEV⌝
            | lit :: lits' =>
                ∃ l : loc, ∃ v_head v_tail,
                ⌜v = #l⌝ ∗ l ↦ PairV v_head v_tail ∗ lit_rel lit v_head ∗ list_rel lits' v_tail
            end%I
        in list_rel lits v
    end%I.

Definition list_rel (lits: list sl_lit) (v: val): iProp Σ :=
    lit_rel (LitList lits) v.

Definition val_rel (v_s: sl_val) (v_h: val): iProp Σ :=
    match v_s with
    | LitVal lit => lit_rel lit v_h
    | ClosureVal params body =>
        ⌜v_h = compile_closurev_val params (compile_expr body)⌝
    end%I.

Definition state_rel (_σ: sl_state): iProp Σ := True%I.

Lemma compile_lit_correct (lit: sl_lit):
    ⊢ WP (compile_lit lit) {{ v, lit_rel lit v }}.
Proof.
induction lit as [b|n|s|lits IHlits] using sl_lit_ind.
- simpl. wp_pures. done.
- simpl. wp_pures. done.
- destruct s as [|c s'].
    + simpl. wp_pures. done.
    + simpl.
        wp_apply (wp_allocN with "[]"); admit.
        (*
        { iPureIntro. simpl. lia. }
        iIntros (l) "Hl".
        wp_pures.
        wp_bind (compile_str #l (String c s')).
        wp_apply (compile_str_correct l (String c s') with "Hl").
        iIntros (_) "Hstr".
        wp_pures.
        iApply wp_value.
        iExists l. iFrame.
        *)
- simpl. admit.
    (*
    revert IHlits.
    induction lits as [|lit lits IH]; iIntros (IHlits).
    + simpl. wp_pures. iApply wp_value. simpl. done.
    + inversion IHlits as [|lit' lits' Hlit Hrest]; subst.
        simpl.
        wp_bind (compile_lit lit).
        wp_apply Hlit.
        iIntros (v_head) "Hhead".
        wp_bind (compile_list (map compile_lit lits)).
        wp_apply (IH Hrest).
        iIntros (v_tail) "Htail".
        wp_pures.
        wp_alloc l as "Hl".
        iApply wp_value.
        iExists l, v_head, v_tail.
        iFrame.
    *)
Admitted.

Lemma compile_expr_correct (e: sl_expr) (v_s: sl_val):
    closed_program e ->
    pure_expr e ->
    sl_terminates e v_s ->
    ⊢ WP (compile_expr e) {{ v_h, val_rel v_s v_h }}.
Admitted.

End correctness.