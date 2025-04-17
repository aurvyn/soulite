#pragma once
#include <memory>
#include <string>
#include <vector>
#include "globals.h"

class ExprAST {
public:
    virtual ~ExprAST() = default;
    virtual std::string to_string() const = 0;
};

template<typename T>
class LiteralExprAST : public ExprAST {
    T val;
public:
    LiteralExprAST(T val) : val(val) {}

    std::string to_string() const override {
        if constexpr (std::is_arithmetic_v<T>) {
            return std::to_string(val);
        } else {
            return '"' + val + '"';
        }
    }
};

class VariableExprAST : public ExprAST {
    std::string name;
public:
    VariableExprAST(const std::string &name) : name(name) {}

    std::string to_string() const override {
        return name;
    }
};

class BinaryExprAST : public ExprAST {
    Token op;
    std::unique_ptr<ExprAST> lhs, rhs;
public:
    BinaryExprAST(Token op, std::unique_ptr<ExprAST> lhs, std::unique_ptr<ExprAST> rhs)
        : op(op), lhs(std::move(lhs)), rhs(std::move(rhs)) {}
    
    std::string to_string() const override {
        std::string operatorStr;
        switch (op) {
            case ASSIGN: operatorStr = "="; break;
            case RANGE: operatorStr = ".."; break;
            case AND: operatorStr = "&&"; break;
            case OR: operatorStr = "||"; break;
            case AMP: operatorStr = "&"; break;
            case PIPE: operatorStr = "|"; break;
            case LT: operatorStr = "<"; break;
            case GT: operatorStr = ">"; break;
            case LE: operatorStr = "<="; break;
            case GE: operatorStr = ">="; break;
            case EQ: operatorStr = "=="; break;
            case NE: operatorStr = "!="; break;
            case SHL: operatorStr = "<<"; break;
            case SHR: operatorStr = ">>"; break;
            case SHLX: operatorStr = "<|"; break;
            case SHRX: operatorStr = "|>"; break;
            case PLUS: operatorStr = "+"; break;
            case DASH: operatorStr = "-"; break;
            case ASTER: operatorStr = "*"; break;
            case DIV: operatorStr = "/"; break;
            case MOD: operatorStr = "%"; break;
            case EXP: operatorStr = "**"; break;
            case DOT: operatorStr = "."; break;
            default: operatorStr = "unknown";
        }
        return "(" + lhs->to_string() + " " + operatorStr + " " + rhs->to_string() + ")";
    }
};

class CallExprAST : public ExprAST {
    std::string callee;
    std::vector<std::unique_ptr<ExprAST>> args;
public:
    CallExprAST(const std::string &callee, std::vector<std::unique_ptr<ExprAST>> args)
        : callee(callee), args(std::move(args)) {}
    
    std::string to_string() const override {
        std::string result = callee + "(";
        for (size_t i = 0; i < args.size(); ++i) {
            result += args[i]->to_string();
            if (i != args.size() - 1) {
                result += " ";
            }
        }
        result += ")";
        return result;
    }
};

class VarExprAST : public ExprAST {
    std::string name;
    bool isMutable;
    std::unique_ptr<ExprAST> value;
public:
    VarExprAST(
        const std::string &name,
        bool isMutable,
        std::unique_ptr<ExprAST> value = nullptr
    ) : name(name), isMutable(isMutable), value(std::move(value)) {}
    
    std::string to_string() const override {
        return (isMutable ? "let mut " : "let ") + name + " = " + value->to_string();
    }
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

    std::string to_string() const {
        std::string result = name + "(";
        for (size_t i = 0; i < args.size(); ++i) {
            result += argTypes[i] + " " + args[i];
            if (i != args.size() - 1) {
                result += ", ";
            }
        }
        result += ") -> " + returnType;
        return result;
    }
};

class FunctionAST {
    std::unique_ptr<PrototypeAST> proto;
    std::unique_ptr<ExprAST> body;
public:
    FunctionAST(std::unique_ptr<PrototypeAST> proto, std::unique_ptr<ExprAST> body)
        : proto(std::move(proto)), body(std::move(body)) {}

    std::string to_string() const {
        return proto->to_string() + " {\n\t" + body->to_string() + "\n}";
    }
};

class ImportAST {
    std::string importName;
    std::vector<std::unique_ptr<ImportAST>> imports;
public:
    ImportAST(
        const std::string &importName,
        std::vector<std::unique_ptr<ImportAST>> imports = {}
    ) : importName(importName), imports(std::move(imports)) {}

    std::string to_string() const {
        std::string result = importName;
        for (const auto &import : imports) {
            result += ":" + import->to_string();
        }
        return result;
    }
};
