From Stdlib Require Import String.
From Stdlib Require Import ZArith.
From Soulite Require Import ast.

Coercion N.of_nat : nat >-> N.
Coercion LitBool: bool >-> sl_lit.
Coercion LitN: N >-> sl_lit.
Coercion LitZ: Z >-> sl_lit.
Coercion LitString: string >-> sl_lit.
Coercion LitVal: sl_lit >-> sl_val.
Coercion ValExpr: sl_val >-> sl_expr.

Declare Custom Entry sl.
Declare Scope sl_scope.
Delimit Scope sl_scope with sl.
Open Scope sl.

Notation "<{ e }>" := e
    (e custom sl, format "'[hv' <{ '/  ' '[v' e ']' '/' }> ']'"): sl_scope.

Notation "- n" := (-n)%Z
    (in custom sl at level 5).
Notation "~ n" := (negb n)
    (in custom sl at level 5).
Infix "." := (BinaryExpr DotOp)
    (in custom sl at level 10, left associativity): sl_scope.
Infix "*" := (BinaryExpr MultOp)
    (in custom sl at level 20, left associativity): sl_scope.
Infix "/" := (BinaryExpr DivOp)
    (in custom sl at level 20, left associativity): sl_scope.
Infix "%" := (BinaryExpr ModOp)
    (in custom sl at level 20, left associativity): sl_scope.
Infix "+" := (BinaryExpr PlusOp)
    (in custom sl at level 30, left associativity): sl_scope.
Infix "-" := (BinaryExpr MinusOp)
    (in custom sl at level 30, left associativity): sl_scope.
Infix "<<" := (BinaryExpr ShiftLOp)
    (in custom sl at level 40, left associativity): sl_scope.
Infix ">>" := (BinaryExpr ShiftROp)
    (in custom sl at level 40, left associativity): sl_scope.
Infix "<|" := (BinaryExpr EndLOp)
    (in custom sl at level 40, left associativity): sl_scope.
Infix "<" := (BinaryExpr LtOp)
    (in custom sl at level 50): sl_scope.
Infix "<=" := (BinaryExpr LeOp)
    (in custom sl at level 50): sl_scope.
Infix ">" := (BinaryExpr GtOp)
    (in custom sl at level 50): sl_scope.
Infix ">=" := (BinaryExpr GeOp)
    (in custom sl at level 50): sl_scope.
Infix "==" := (BinaryExpr EqOp)
    (in custom sl at level 50): sl_scope.
Infix "!=" := (BinaryExpr NotEqOp)
    (in custom sl at level 50): sl_scope.
Infix "&&" := (BinaryExpr AndOp)
    (in custom sl at level 60, left associativity): sl_scope.
Infix "||" := (BinaryExpr OrOp)
    (in custom sl at level 60, left associativity): sl_scope.


Notation "expr" := expr
    (in custom sl at level 0, expr constr at level 0).
Notation "f ( a .. z )" := (CallExpr f (cons a .. (cons z nil) ..))
    (in custom sl at level 0).
Notation "[< a .. z >]" := (ListExpr (cons a .. (cons z nil) ..))
    (in custom sl at level 0).
Notation "[ type ]" := (TypeList type)
    (in custom sl at level 70).
Notation "{ typeA .. typeZ -> ret_typeA .. ret_typeZ }" :=
    (TypeClosure
        (cons typeA .. (cons typeZ nil) ..)
        (cons ret_typeA .. (cons ret_typeZ nil) ..))
    (in custom sl at level 75).
Notation "if_true <- cond ; if_false" := (TernaryExpr cond if_true if_false)
    (in custom sl at level 80, left associativity).
Notation "name ,= expr" := (AssignExpr name expr)
    (in custom sl at level 85, right associativity).
Notation "name : type = expr" := (DeclareExpr name false type expr)
    (in custom sl at level 90, right associativity).
Notation "name ; type = expr" := (DeclareExpr name true type expr)
    (in custom sl at level 90, right associativity).
Notation "( expr )" := expr
    (in custom sl, expr at level 95).
Notation "f paramA .. paramZ : typeA .. typeZ -> ret_typeA .. ret_typeZ '\n' expr" :=
    {|
        name := f;
        params := cons paramA .. (cons paramZ nil) ..;
        param_types := cons typeA .. (cons typeZ nil) ..;
        return_types := cons ret_typeA .. (cons ret_typeZ nil) ..;
        body := expr
    |}
    (in custom sl at level 98).
Notation "'\t' exprA '\n\t' .. '\n\t' exprZ '\n'" := (SeqExpr (cons exprA .. (cons exprZ nil) ..))
    (in custom sl at level 99).