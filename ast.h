#include <memory>
#include <string>
#include <vector>
#include "globals.h"

class ExprAST {
public:
    virtual ~ExprAST() = default;
};

template<typename T>
class LiteralExprAST : public ExprAST {
    T val;
public:
    LiteralExprAST(T val) : val(val) {}
};

class VariableExprAST : public ExprAST {
    std::string name;
public:
    VariableExprAST(const std::string &name) : name(name) {}
};

class BinaryExprAST : public ExprAST {
    Token op;
    std::unique_ptr<ExprAST> lhs, rhs;
public:
    BinaryExprAST(Token op, std::unique_ptr<ExprAST> lhs, std::unique_ptr<ExprAST> rhs)
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
    std::vector<std::string> argTypes;
    std::vector<std::string> args;
    std::string returnType;
public:
    PrototypeAST(
        const std::string &name,
        std::vector<std::string> argTypes,
        std::vector<std::string> args,
        std::string returnType
    ) : name(name),
        argTypes(std::move(argTypes)),
        args(std::move(args)),
        returnType(std::move(returnType))
    {}

    const std::string &getName() const { return name; }
};

class FunctionAST {
    std::unique_ptr<PrototypeAST> proto;
    std::unique_ptr<ExprAST> body;
public:
    FunctionAST(std::unique_ptr<PrototypeAST> proto, std::unique_ptr<ExprAST> body)
        : proto(std::move(proto)), body(std::move(body)) {}
};
