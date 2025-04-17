#include <cctype>
#include <cstdio>
#include <string>
#include "globals.h"

static std::string current_token;
static float current_float;
static int current_int;

static Token getToken() {
    static char lastChar = ' ';
    current_token.clear();
    
    // Skip whitespace
    while (isspace(lastChar)) {
        lastChar = getchar();
    }

    if (lastChar == '!') {
        lastChar = getchar();
        if (lastChar == '=') return NE;
        return EXCL;
    }
    if (lastChar == '-') {
        lastChar = getchar();
        if (lastChar == '=') return MINUS_EQ;
        if (lastChar == '-') return DEC;
        if (lastChar == '>') return ARROW;
        return DASH;
    }
    if (lastChar == '+') {
        lastChar = getchar();
        if (lastChar == '=') return PLUS_EQ;
        if (lastChar == '+') return INC;
        return PLUS;
    }
    if (lastChar == '/') {
        lastChar = getchar();
        if (lastChar == '=') return DIV_EQ;
        return DIV;
    }
    if (lastChar == '*') {
        lastChar = getchar();
        if (lastChar == '=') return MUL_EQ;
        if (lastChar == '*') {
            lastChar = getchar();
            return lastChar == '=' ? EXP_EQ : EXP;
        }
        return ASTER;
    }
    if (lastChar == '%') {
        lastChar = getchar();
        if (lastChar == '=') return MOD_EQ;
        return MOD;
    }
    if (lastChar == '(') return LPAREN;
    if (lastChar == ')') return RPAREN;
    if (lastChar == '{') return LBRACE;
    if (lastChar == '}') return RBRACE;
    if (lastChar == '[') return LBRACK;
    if (lastChar == ']') return RBRACK;
    if (lastChar == ',') return COMMA;
    if (lastChar == ':') return COLON;
    if (lastChar == '.') {
        lastChar = getchar();
        if (lastChar == '.') return RANGE;
        return DOT;
    }
    if (lastChar == '&') {
        lastChar = getchar();
        if (lastChar == '=') return AND_EQ;
        if (lastChar == '&') return AND;
        return AMP;
    }
    if (lastChar == '|') {
        lastChar = getchar();
        if (lastChar == '=') return OR_EQ;
        if (lastChar == '|') return OR;
        if (lastChar == '>') return SHRX;
        return PIPE;
    }
    if (lastChar == '~') {
        lastChar = getchar();
        if (lastChar == '=') return INV_EQ;
        return TILDE;
    }
    if (lastChar == '^') {
        lastChar = getchar();
        if (lastChar == '=') return XOR_EQ;
        return XOR;
    }
    if (lastChar == '<') {
        lastChar = getchar();
        if (lastChar == '=') return LE;
        if (lastChar == '|') {
            lastChar = getchar();
            return lastChar == '=' ? SHLX_EQ : SHLX;
        }
        if (lastChar == '<') {
            lastChar = getchar();
            return lastChar == '=' ? SHL_EQ : SHL;
        }
        return LT;
    }
    if (lastChar == '>') {
        lastChar = getchar();
        if (lastChar == '=') return GE;
        if (lastChar == '>') return SHR;
        return GT;
    }
    if (lastChar == '=') {
        lastChar = getchar();
        if (lastChar == '=') return EQ;
        return ASSIGN;
    }
    if (lastChar == '#') return POUND;
    if (lastChar == '@') return AT;
    if (lastChar == '$') return DOLLAR;
    if (lastChar == '\'') return APOSTROPHE;

    // Tokenize Comments
    if (lastChar == ';') {
        lastChar = getchar();
        while (lastChar != EOF && lastChar != '\n' && lastChar != '\r') {
            current_token += lastChar;
            lastChar = getchar();
        }
        return COMMENT;
    }

    // Identifier: [a-z][a-zA-Z0-9]*
    // Type: [A-Z][a-zA-Z0-9]*
    if (isalpha(lastChar)) {
        Token tok = islower(lastChar) ? IDENTIFIER : TYPE;
        current_token = lastChar;
        while (isalnum((lastChar = getchar()))) {
            current_token += lastChar;
        }
        return tok;
    }

    // Number: [0-9.]+
    if (isdigit(lastChar) || lastChar == '.') {
        int periods = lastChar == '.';
        do {
            current_token += lastChar;
            lastChar = getchar();
            periods += lastChar == '.';
        } while (isdigit(lastChar) || lastChar == '.');
        if (periods > 1) {
            logError("Invalid number format");
            return INVALID;
        } else if (periods == 1) {
            current_float = std::stof(current_token);
            return FLOAT;
        } else {
            current_int = std::stoi(current_token);
            return INT;
        }
    }

    // String literal: ".*"
    if (lastChar == '"') {
        lastChar = getchar();
        while (lastChar != EOF && lastChar != '"') {
            current_token += lastChar;
            lastChar = getchar();
        }
        if (lastChar == '"') {
            lastChar = getchar();
            return STRING;
        } else {
            logError("String literal does not have a closing quote");
            return INVALID;
        }
    }

    // End of file
    if (lastChar == EOF) {
        logError("End of file reached");
        return EoF;
    }

    // Any other character
    logError("Invalid token");
    return INVALID;
}
