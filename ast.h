#include <memory>
#include <string>
#include <vector>
#include "globals.h"

enum UniOp {
    NOT,   // !
    NEG,   // -
    NOT_B, // ~
    INC,   // ++
    DEC,   // --
    REF,   // &
    DEREF, // *
};

enum BiOp {
    RANGE = 0, // ..

    AND =   10, // &&
    OR =    11, // ||

    AND_B = 20, // &
    XOR_B = 21, // -|
    OR_B =  22, // |

    LT =    30, // <
    GT =    31, // >
    LE =    32, // <=
    GE =    33, // >=
    EQ =    34, // ==
    NE =    35, // !=

    SHL =   40, // <<
    SHR =   41, // >>
    SHLX =  42, // <|
    SHRX =  43, // |>

    ADD =   50, // +
    SUB =   51, // -

    MUL =   60, // *
    DIV =   61, // /
    MOD =   62, // %

    EXP =   70, // ^

    DOT =   80, // .
};

class ExprAST {
public:
    virtual ~ExprAST() = default;
};

template<typename T>
class ValueExprAST : public ExprAST {
    T val;
public:
    ValueExprAST(T val) : val(val) {}
};

class VariableExprAST : public ExprAST {
    std::string name;
public:
    VariableExprAST(const std::string &name) : name(name) {}
};

class BinaryExprAST : public ExprAST {
    BiOp op;
    std::unique_ptr<ExprAST> lhs, rhs;
public:
    BinaryExprAST(BiOp op, std::unique_ptr<ExprAST> lhs, std::unique_ptr<ExprAST> rhs)
        : op(op), lhs(std::move(lhs)), rhs(std::move(rhs)) {}
};

class CallExprAST : public ExprAST {
    std::string callee;
    std::vector<std::unique_ptr<ExprAST>> args;
public:
    CallExprAST(const std::string &callee, std::vector<std::unique_ptr<ExprAST>> args)
        : callee(callee), args(std::move(args)) {}
};

class PrototypeAST {
    std::string name;
    std::vector<std::string> args;
public:
    PrototypeAST(const std::string &name, std::vector<std::string> args)
        : name(name), args(std::move(args)) {}

    const std::string &getName() const { return name; }
};

class FunctionAST {
    std::unique_ptr<PrototypeAST> proto;
    std::unique_ptr<ExprAST> body;
public:
    FunctionAST(std::unique_ptr<PrototypeAST> proto, std::unique_ptr<ExprAST> body)
        : proto(std::move(proto)), body(std::move(body)) {}
};
