#include <memory>
#include <string>
#include <vector>
#include "ast.h"
#include "lexer.h"

static std::unique_ptr<ExprAST> parseExpression();

static int current_token;
static int getNextToken() {
    return current_token = gettok();
}

inline std::unique_ptr<ExprAST> logError(const char *str) {
    fprintf(stderr, "Error: %s\n", str);
    return nullptr;
}

inline std::unique_ptr<PrototypeAST> logErrorP(const char *str) {
    logError(str);
    return nullptr;
}

template<typename T>
static std::unique_ptr<ExprAST> parseValueExpr() {
    std::unique_ptr<ExprAST> result;
    if (std::is_same<T, float>::value) {
        result = std::make_unique<ValueExprAST<float>>(f_num);
    } else if (std::is_same<T, int>::value) {
        result = std::make_unique<ValueExprAST<int>>(i_num);
    } else {
        logError("Unsupported type for value expression");
        return nullptr;
    }
    getNextToken(); // consume the number
    return result;
};

static std::unique_ptr<ExprAST> parseParenExpr() {
    getNextToken(); // consume '('
    auto expr = parseExpression();
    if (!expr) return nullptr;
    if (current_token != ')') logError("Expected ')'");
    getNextToken(); // consume ')'
    return expr;
}

static std::unique_ptr<ExprAST> parseIdentifierExpr() {
    std::string idName = str;
    getNextToken(); // consume identifier
    if (current_token != '(') return std::make_unique<VariableExprAST>(idName);
    
    getNextToken(); // consume '('
    std::vector<std::unique_ptr<ExprAST>> args;
    if (current_token != ')') {
        while (true) {
            if (auto arg = parseExpression()) {
                args.push_back(std::move(arg));
            } else {
                return nullptr;
            }
            if (current_token == ')') break;
        }
    }
    
    getNextToken(); // consume ')'
    return std::make_unique<CallExprAST>(idName, std::move(args));
}

static std::unique_ptr<ExprAST> parsePrimary() {
    switch (current_token) {
        case IDENTIFIER:
            return parseIdentifierExpr();
        case FLOAT:
            return parseValueExpr<float>();
        case INT:
            return parseValueExpr<int>();
        case '(':
            return parseParenExpr();
        default:
            return logError("Unknown token when expecting an expression");
    }
}

static int binopPrecedence(BiOp op) {
    return static_cast<int>(op) / 10 * 10;
};

static int getTokPrecedence() {
    if (!isascii(current_token)) return -1;
    int tokPrec = binopPrecedence(RANGE); // fix this later, have to find out what the binary operator is through lexer
    if (tokPrec <= 0) return -1;
    return tokPrec;
}

static std::unique_ptr<ExprAST> ParseBinOpRHS(int exprPrec, std::unique_ptr<ExprAST> lhs) {
    while (true) {
        int tokPrec = getTokPrecedence();
        if (tokPrec < exprPrec) return lhs;
        
        BiOp binOp = RANGE; // fix this later, have to find out what the binary operator is through lexer
        getNextToken(); // consume binop
        
        auto rhs = parsePrimary();
        if (!rhs) return nullptr;
        
        int nextPrec = getTokPrecedence();
        if (tokPrec < nextPrec) {
            rhs = ParseBinOpRHS(tokPrec + 1, std::move(rhs));
            if (!rhs) return nullptr;
        }
        
        lhs = std::make_unique<BinaryExprAST>(binOp, std::move(lhs), std::move(rhs));
    }
}

static std::unique_ptr<ExprAST> parseExpression() {
    auto lhs = parsePrimary();
    if (!lhs) return nullptr;
    
    return ParseBinOpRHS(0, std::move(lhs));
}
