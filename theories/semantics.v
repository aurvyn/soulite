From Stdlib Require Import ZArith.
From stdpp Require Import gmap.
From Soulite Require Import ast.
From Soulite Require Import notation.

Definition heap := gmap Z sl_val.
Fixpoint heap_array (l: Z) (vals: list sl_val): heap :=
    match vals with
    | [] => gmap_empty
    | val :: vals' => union {[l := val]} (heap_array (l + 1) vals')
    end.

Inductive sl_step : sl_expr * heap -> sl_expr * heap -> Prop :=
| BetaS x e1 e2 e' σ :
     is_val e2 ->
     e' = subst' x e2 e1 ->
     sl_step (App (Lam x e1) e2, σ) (e', σ)
| TBetaS e1 σ :
      sl_step (TApp (TLam e1), σ) (e1, σ)
| UnpackS e1 e2 e' x σ :
      is_val e1 ->
      e' = subst' x e1 e2 ->
      sl_step (Unpack x (Pack e1) e2, σ) (e', σ)
| UnOpS op e v v' σ :
     to_val e = Some v ->
     un_op_eval op v = Some v' ->
     sl_step (UnOp op e, σ) (of_val v', σ)
| BinOpS op e1 v1 e2 v2 v' σ :
     to_val e1 = Some v1 ->
     to_val e2 = Some v2 ->
     bin_op_eval op v1 v2 = Some v' ->
     sl_step (BinOp op e1 e2, σ) (of_val v', σ)
| IfTrueS e1 e2 σ :
     sl_step (If (Lit (LitBool true)) e1 e2, σ) (e1, σ)
| IfFalseS e1 e2 σ :
     sl_step (If (Lit (LitBool false)) e1 e2, σ) (e2, σ)
| FstS e1 e2 σ :
     is_val e1 ->
     is_val e2 ->
     sl_step (Fst (Pair e1 e2), σ) (e1, σ)
| SndS e1 e2 σ :
     is_val e1 ->
     is_val e2 ->
     sl_step (Snd (Pair e1 e2), σ) (e2, σ)
| CaseLS e e1 e2 σ :
     is_val e ->
     sl_step (Case (InjL e) e1 e2, σ) (App e1 e, σ)
| CaseRS e e1 e2 σ :
     is_val e ->
     sl_step (Case (InjR e) e1 e2, σ) (App e2 e, σ)
| UnrollS e σ :
      is_val e ->
      sl_step (Unroll (Roll e), σ) (e, σ)
| NewS e v σ l :
     σ !! l = None ->
     to_val e = Some v ->
     sl_step (New e, σ) (Lit $ LitLoc l, init_heap l 1 v σ)
| LoadS l v σ :
     σ !! l = Some v ->
     sl_step (Load (Lit $ LitLoc l), σ) (of_val v, σ)
| StoreS l v w e2 σ :
     σ !! l = Some v ->
     to_val e2 = Some w ->
     sl_step (Store (Lit $ LitLoc l) e2, σ)
               (Lit LitUnit, <[l:=w]> σ)
.