From Soulite Require Export ast.

Coercion NExpr: nat >-> soulite_expr.
Coercion ZExpr: Z >-> soulite_expr.
Coercion StringExpr: string >-> soulite_expr.

Declare Scope soulite_scope.
Delimit Scope soulite_scope with sl.
Bind Scope soulite_scope with soulite_expr.
Open Scope soulite_scope.

Infix "." := (BinaryExpr DotOp) (at level 39, left associativity): soulite_scope.
Infix "*" := (BinaryExpr MultOp): soulite_scope.
Infix "/" := (BinaryExpr DivOp): soulite_scope.
Infix "+" := (BinaryExpr PlusOp): soulite_scope.
Infix "-" := (BinaryExpr MinusOp): soulite_scope.
Infix "<<" := (BinaryExpr ShiftLeftOp) (at level 51, left associativity): soulite_scope.
Infix "<|" := (BinaryExpr EndLeftOp) (at level 51, left associativity): soulite_scope.
Infix "<" := (BinaryExpr LtOp): soulite_scope.
Infix "<=" := (BinaryExpr LteOp): soulite_scope.
Infix ">" := (BinaryExpr GtOp): soulite_scope.
Infix ">=" := (BinaryExpr GteOp): soulite_scope.
Infix "==" := (BinaryExpr EqOp) (at level 70): soulite_scope.
Infix "!=" := (BinaryExpr NotEqOp) (at level 70): soulite_scope.
Infix "&&" := (BinaryExpr AndOp): soulite_scope.
Infix "||" := (BinaryExpr OrOp): soulite_scope.
Infix "=" := AssignExpr: soulite_scope.


Notation "if_true <- cond ; if_false" := (TernaryExpr cond if_true if_false)
    (at level 75).
Notation "name : type = expr" := (DeclareExpr name false type expr)
    (at level 80).
Notation "name ; type = expr" := (DeclareExpr name true type expr)
    (at level 80).