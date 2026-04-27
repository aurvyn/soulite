From Stdlib Require Import String.
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
    | ShiftLOp => BinOp heap_lang.ShiftLOp rhs lhs
    | ShiftROp => BinOp heap_lang.ShiftROp rhs lhs
    end%E.

(* Crash the program if it managed to reach here *)
Definition unreachable := #0 #0.

Fixpoint compile_call_closure (lam: expr) (args: list expr): expr :=
    match lam with
    | Lam param body => match param, args with
        | BAnon, [] => lam #0
        | BNamed _, [arg] => lam arg
        | BNamed _, arg :: args' => compile_call_closure (lam arg) args'
        | _, _ => unreachable
        end
    | _ => unreachable
    end.

Fixpoint compile_expr (e: sl_expr): expr :=
    match e with
    | ValExpr v => match v with
        | LitVal lit => match lit with
            | LitBoolean b => Val #b
            | LitZ n => Val #n
            | LitString str => AllocN #1 (Val (LitV (LitInt (String.length str))))
            | LitList vals => AllocN #(length vals) #0
            end
        | ClosureVal params body => Val match params with
            | [] => LamV BAnon (compile_expr body)
            | [param] => LamV param (compile_expr body)
            | param :: params' => LamV param (compile_expr (ValExpr (ClosureVal params' body)))
            end
        end
    | VarExpr name => Var name
    | ListExpr exprs => AllocN #(length exprs) #0
    | UnaryExpr op expr => UnOp (compile_unop op) (compile_expr expr)
    | BinaryExpr op lhs rhs => (compile_binop op (compile_expr lhs) (compile_expr rhs))
    | <{if_true <- cond ;; if_false}> => If (compile_expr cond) (compile_expr if_true) (compile_expr if_false)
    | CallClosureExpr closure params => compile_call_closure (compile_expr closure) (rev (map compile_expr params))
    | CallFunctionExpr name params => compile_call_closure (compile_expr name) (rev (map compile_expr params))
    | <{name : type = expr}> => Lam name (compile_expr expr) #0
    | <{name ; type = expr}> => AllocN #(alloc_length expr) (compile_expr expr)
    | <{name ,= expr}> => Store name (compile_expr expr)
    | ClosureExpr params body => match params with
        | [] => Lam BAnon (compile_expr body)
        | [param] => Lam param (compile_expr body)
        | param :: params' => Lam param (compile_expr (ClosureExpr params' body))
        end
    | WhileExpr cond body => If (compile_expr cond) (compile_expr body) #0
    | SeqExpr e1 e2 => (compile_expr e1) ;; (compile_expr e2)
    end.