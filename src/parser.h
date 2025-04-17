#include <memory>
#include <string>
#include <vector>
#include "ast.h"
#include "globals.h"
#include "lexer.h"

static Token current_token_type;

static Token getNextToken() {
    current_token_type = getToken();
    return current_token_type;
}

static std::unique_ptr<ExprAST> parseExpression();

static std::unique_ptr<ExprAST> parseFloatExpr() {
    if (current_token_type != FLOAT) {
        logError("Expected a float expression");
        return nullptr;
    }
    auto result = std::make_unique<LiteralExprAST<float>>(current_float);
    getNextToken(); // consume float
    return result;
};

static std::unique_ptr<ExprAST> parseIntExpr() {
    if (current_token_type != INT) {
        logError("Expected an integer expression");
        return nullptr;
    }
    auto result = std::make_unique<LiteralExprAST<int>>(current_int);
    getNextToken(); // consume integer
    return result;
}

static std::unique_ptr<ExprAST> parseStringExpr() {
    if (current_token_type != STRING) {
        logError("Expected a string expression");
        return nullptr;
    }
    auto result = std::make_unique<LiteralExprAST<std::string>>(current_token);
    getNextToken(); // consume string
    return result;
}

static std::unique_ptr<ExprAST> parseTypeExpr() {
    if (current_token_type != TYPE) {
        logError("Expected a type expression");
        return nullptr;
    }
    auto result = std::make_unique<LiteralExprAST<std::string>>(current_token);
    getNextToken(); // consume type
    return result;
}

static std::unique_ptr<ExprAST> parseParenExpr() {
    getNextToken(); // consume '('
    auto expr = parseExpression();
    if (!expr) return nullptr;
    if (current_token_type != RPAREN) logError("Expected `)`");
    getNextToken(); // consume ')'
    return expr;
}

static std::unique_ptr<ExprAST> parseIdentifierExpr() {
    std::string idName = current_token;
    getNextToken(); // consume identifier
    if (current_token_type != LPAREN) return std::make_unique<VariableExprAST>(idName);
    
    getNextToken(); // consume '('
    std::vector<std::unique_ptr<ExprAST>> args;
    while (current_token_type != RPAREN) {
        if (auto arg = parseExpression()) {
            args.push_back(std::move(arg));
        } else {
            return nullptr;
        }
    }
    
    getNextToken(); // consume ')'
    return std::make_unique<CallExprAST>(idName, std::move(args));
}

static std::unique_ptr<ExprAST> parsePrimary() {
    switch (current_token_type) {
        case IDENTIFIER:
            return parseIdentifierExpr();
        case FLOAT:
            return parseFloatExpr();
        case INT:
            return parseIntExpr();
        case STRING:
            return parseStringExpr();
        case LPAREN:
            return parseParenExpr();
        default:
            logError("Unknown token when expecting an expression");
            return nullptr;
    }
}

static int binopPrecedence(Token op) {
    return static_cast<int>(op) / 10 * 10;
};

static int getTokPrecedence() {
    int tokPrec = binopPrecedence(current_token_type);
    if (tokPrec > 80) return -1;
    return tokPrec;
}

static std::unique_ptr<ExprAST> ParseBinOpRHS(int exprPrec, std::unique_ptr<ExprAST> lhs) {
    while (true) {
        int tokPrec = getTokPrecedence();
        if (tokPrec < exprPrec) return lhs;
        
        Token binOp = current_token_type;
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

static std::unique_ptr<PrototypeAST> parsePrototype() {
    if (current_token_type != IDENTIFIER) {
        logError("Expected function name in prototype");
        return nullptr;
    }
    
    std::string fnName = current_token;
    getNextToken(); // consume function name
    
    if (current_token_type != PIPE) {
        logError("Expected `|` in prototype");
        return nullptr;
    }
    
    std::vector<std::string> argTypes;
    std::string returnType;
    while (getNextToken() == TYPE) {
        argTypes.push_back(current_token);
    }
    
    if (current_token_type == ARROW) {
        if (getNextToken() == TYPE) {
            returnType = current_token;
        } else {
            logError("Expected return type in prototype");
            return nullptr;
        }
    }
    
    std::vector<std::string> args;
    for (int i = 0; i < argTypes.size(); ++i) {
        if (current_token_type != APOSTROPHE && current_token_type != COMMA) {
            logError("Expected `'` or `,` in argument list");
            return nullptr;
        }
        getNextToken(); // consume APOS or COMMA
        if (current_token_type != IDENTIFIER) {
            logError("Expected argument name");
            return nullptr;
        }
        args.push_back(current_token);
        getNextToken(); // consume argument name
    }

    if (current_token_type != ASSIGN) {
        logError("Expected `=` in prototype");
        return nullptr;
    }
    getNextToken(); // consume '='
    
    return std::make_unique<PrototypeAST>(
        fnName,
        std::move(argTypes),
        std::move(args),
        std::move(returnType)
    );
}

static std::unique_ptr<FunctionAST> parseDefinition() {
    getNextToken(); // consume '.'
    
    auto proto = parsePrototype();
    if (!proto) return nullptr;
    
    if (auto body = parseExpression()) {
        return std::make_unique<FunctionAST>(std::move(proto), std::move(body));
    }
    return nullptr;
}

static std::unique_ptr<FunctionAST> parseTopLevelExpr() {
    if (auto expr = parseExpression()) {
        auto proto = std::make_unique<PrototypeAST>("", std::vector<std::string>(), std::vector<std::string>(), "");
        return std::make_unique<FunctionAST>(std::move(proto), std::move(expr));
    }
    return nullptr;
}

static void mainLoop() {
    while (true) {
        fprintf(stderr, ">>> ");
        switch (current_token_type) {
            case EoF:
                return;
            case COMMENT:
                continue;
            case DOT:
                parseDefinition();
                break;
            default:
                parseTopLevelExpr();
                break;
        }
    }
}
