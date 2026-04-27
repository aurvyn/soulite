From Stdlib Require Import String Ascii.
From iris.heap_lang Require Import notation.
From Soulite Require Import ast notation semantics.

Definition compile_unop (op: unop): un_op :=
    match op with
    | NegOp => MinusUnOp
    | NotOp => heap_lang.NegOp
    end.

Definition compile_binop (op: binop) (lhs rhs: expr): expr :=
    match op with
    | PlusOp => lhs + rhs
    | MinusOp => lhs - rhs
    | MultOp => lhs * rhs
    | DivOp => BinOp QuotOp lhs rhs
    | ModOp => BinOp RemOp lhs rhs
    | LtOp => lhs < rhs
    | LeOp => BinOp heap_lang.LeOp lhs rhs
    | GtOp => BinOp heap_lang.LeOp rhs lhs
    | GeOp => rhs < lhs
    | EqOp => lhs = rhs
    | NotEqOp => ~(lhs = rhs)
    | AndOp => lhs && rhs
    | OrOp => lhs || rhs
    | ShiftLOp => BinOp heap_lang.ShiftLOp lhs rhs
    | ShiftROp => BinOp heap_lang.ShiftROp lhs rhs
    end%E.

Fixpoint compile_str (l: expr) (str: string): expr :=
    match str with
    | EmptyString => #()
    | String c EmptyString => Store l #(Z.of_N (N_of_ascii c))
    | String c s => Store l #(Z.of_N (N_of_ascii c));; compile_str (BinOp OffsetOp l #1) s
    end.

Fixpoint compile_list (exprs: list expr): expr :=
    match exprs with
    | [] => #()
    | e :: es => Alloc (e, compile_list es)
    end.

Fixpoint compile_lit (lit: sl_lit): expr :=
    match lit with
    | LitBoolean b => #b
    | LitZ n => #n
    | LitString "" => #() (* disallow empty strings *)
    | LitString str => let: "loc" := AllocN #(String.length str) #0 in
        compile_str "loc" str;; "loc"
    | LitList lits => compile_list (map compile_lit lits)
    end.

Fixpoint compile_call_closure (lam: expr) (args: list expr): expr :=
    match args with
    | [] => lam #() (* for closures with zero params *)
    | [arg] => lam arg
    | arg :: args' => compile_call_closure (lam arg) args'
    end.

Fixpoint compile_closure (params: list string) (body: expr): expr :=
    match params with
    | [] => Lam BAnon body
    | [param] => Lam param body
    | param :: params' => Lam param (compile_closure params' body)
    end.

Fixpoint compile_closurev (params: list string) (body: expr): expr :=
    Val match params with
    | [] => LamV BAnon body
    | [param] => LamV param body
    | param :: params' => LamV param (compile_closurev params' body)
    end.

Fixpoint compile_expr (e: sl_expr): expr :=
    match e with
    | ValExpr v => match v with
        | LitVal lit => compile_lit lit
        | ClosureVal params body => compile_closurev params (compile_expr body)
        end
    | <{!name}> => Var name
    | ListExpr exprs => compile_list (map compile_expr exprs)
    | UnaryExpr op expr => UnOp (compile_unop op) (compile_expr expr)
    | BinaryExpr op lhs rhs => (compile_binop op (compile_expr lhs) (compile_expr rhs))
    | <{if_true <- cond;; if_false}> => if: compile_expr cond then compile_expr if_true else compile_expr if_false
    | CallClosureExpr closure params => compile_call_closure (compile_expr closure) (rev (map compile_expr params))
 (* | CallFunctionExpr name params => compile_call_closure name (rev (map compile_expr params)) *)
    | <{name; type = expr}> => ref (compile_expr expr) (* end of sequence *)
    | <{name: type = expr}> => compile_expr expr (* end of sequence *)
    | <{name ,= expr}> => Store name (compile_expr expr)
    | ClosureExpr params body => compile_closure params (compile_expr body)
    | WhileExpr cond body => (rec: "while" <> :=
        if: compile_expr cond then
            compile_expr body;; "while" #()
        else #()) #()
    | SeqExpr e1 e2 => let e2' := compile_expr e2 in
        match e1 with (* assume that declarations only happen at top-level seq expr *)
        | <{name; type = expr}> => let: name := ref (compile_expr expr) in e2'
        | <{name: type = expr}> => let: name := compile_expr expr in e2'
        | _ => compile_expr e1;; e2'
        end
    end.