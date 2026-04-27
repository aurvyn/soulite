From Stdlib Require Import String ZArith.
From Soulite Require Import ast.

Coercion Z.of_nat: nat >-> Z.
Coercion LitBoolean: bool >-> sl_lit.
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
Notation "! var" := (VarExpr var)
    (in custom sl at level 5, format "! var").
Notation "- expr" := (UnaryExpr NegOp expr)
    (in custom sl at level 5, format "- expr").
Notation "~ expr" := (UnaryExpr NotOp expr)
    (in custom sl at level 5, format "~ expr").
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
Notation "closure ( a .. z )" := (CallClosureExpr closure (cons a .. (cons z nil) ..))
    (in custom sl at level 0, format "closure ( a  ..  z )").
Notation "[< a .. z >]" := (ListExpr (cons a .. (cons z nil) ..))
    (in custom sl at level 0, format "[< a  ..  z >]").
(* Notation "! f ( a .. z )" := (CallFunctionExpr f (cons a .. (cons z nil) ..))
    (in custom sl at level 5, format "! f ( a  ..  z )"). *)
Notation "[ type ]" := (TypeList type)
    (in custom sl at level 70, format "[ type ]").
Notation "{ typeA .. typeZ -> ret_typeA .. ret_typeZ }" :=
    (TypeClosure
        (cons typeA .. (cons typeZ nil) ..)
        (cons ret_typeA .. (cons ret_typeZ nil) ..))
    (in custom sl at level 75, format "{ typeA  ..  typeZ  ->  ret_typeA  ..  ret_typeZ }").
Notation "if_true <- cond ;; if_false" := (TernaryExpr cond if_true if_false)
    (in custom sl at level 80, left associativity).
Infix ",=" := AssignExpr
    (in custom sl at level 85, right associativity).
Notation "name : type = expr" := (DeclareExpr name false type expr)
    (in custom sl at level 90, right associativity, format "name :  type  =  expr").
Notation "name ; type = expr" := (DeclareExpr name true type expr)
    (in custom sl at level 90, right associativity, format "name ;  type  =  expr").
Notation "( expr )" := expr
    (in custom sl, expr at level 95, format "( expr )").
Notation "f paramA .. paramZ : typeA .. typeZ -> ret_typeA .. ret_typeZ \n \t expr \n" :=
    {|
        name := f;
        params := cons paramA .. (cons paramZ nil) ..;
        param_types := cons typeA .. (cons typeZ nil) ..;
        return_types := cons ret_typeA .. (cons ret_typeZ nil) ..;
        body := expr
    |}
    (in custom sl at level 200, format
        "f  paramA  ..  paramZ :  typeA  ..  typeZ  ->  ret_typeA  ..  ret_typeZ \n '//' \t   expr \n '//'").
Notation "exprA \n \t exprB" := (SeqExpr exprA exprB)
    (in custom sl at level 100, exprB at level 200, format "exprA \n '//' \t   exprB").

(* Examples: *)
Open Scope string_scope.
Compute {|
    name := "func";
    params := cons "a" (cons "b" nil);
    param_types := cons TypeZ (cons TypeZ nil);
    return_types := cons TypeZ (cons TypeZ nil);
    body := <{"A" : TypeZ = -1 \n\t "1" }>
|}.