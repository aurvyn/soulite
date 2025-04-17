#pragma once
#include <cctype>
#include <cstdio>
#include <string>
#include <fstream>
#include "globals.h"

static std::ifstream file;
static std::string current_token;
static float current_float;
static int current_int;

#define CONSUME_RETURN(chr, token) if (lastChar == chr) { \
    current_token = lastChar; \
    file.get(lastChar); \
    return token; \
}

static Token getToken() {
    static char lastChar = ' ';
    current_token.clear();

    // End of file
    if (lastChar == EOF) {
        printf("End of file reached\n");
        file.close();
        return EoF;
    }
    
    // Skip whitespace
    while (file && isspace(lastChar)) {
        file.get(lastChar);
        if (file.eof()) {
            return EoF;
        }
    }

    if (lastChar == '!') {
        file.get(lastChar);
        CONSUME_RETURN('=', NE);
        return EXCL;
    }
    if (lastChar == '-') {
        file.get(lastChar);
        CONSUME_RETURN('=', MINUS_EQ);
        CONSUME_RETURN('-', NE);
        CONSUME_RETURN('>', ARROW);
        return DASH;
    }
    if (lastChar == '+') {
        file.get(lastChar);
        CONSUME_RETURN('=', PLUS_EQ);
        CONSUME_RETURN('+', INC);
        return PLUS;
    }
    if (lastChar == '/') {
        file.get(lastChar);
        CONSUME_RETURN('=', DIV_EQ);
        return DIV;
    }
    if (lastChar == '*') {
        file.get(lastChar);
        CONSUME_RETURN('=', MUL_EQ);
        if (lastChar == '*') {
            file.get(lastChar);
            CONSUME_RETURN('=', EXP_EQ);
            return EXP;
        }
        return ASTER;
    }
    if (lastChar == '%') {
        file.get(lastChar);
        CONSUME_RETURN('=', MOD_EQ);
        return MOD;
    }
    CONSUME_RETURN('(', LPAREN);
    CONSUME_RETURN(')', RPAREN);
    CONSUME_RETURN('{', LBRACE);
    CONSUME_RETURN('}', RBRACE);
    CONSUME_RETURN('[', LBRACK);
    CONSUME_RETURN(']', RBRACK);
    CONSUME_RETURN(',', COMMA);
    CONSUME_RETURN(':', COLON);
    if (lastChar == '.') {
        file.get(lastChar);
        CONSUME_RETURN('.', RANGE);
        return DOT;
    }
    if (lastChar == '&') {
        file.get(lastChar);
        CONSUME_RETURN('=', AND_EQ);
        CONSUME_RETURN('&', AND);
        return AMP;
    }
    if (lastChar == '|') {
        file.get(lastChar);
        CONSUME_RETURN('=', OR_EQ);
        CONSUME_RETURN('|', OR);
        CONSUME_RETURN('>', SHRX);
        return PIPE;
    }
    if (lastChar == '~') {
        file.get(lastChar);
        CONSUME_RETURN('=', INV_EQ);
        return TILDE;
    }
    if (lastChar == '^') {
        file.get(lastChar);
        CONSUME_RETURN('=', XOR_EQ);
        return XOR;
    }
    if (lastChar == '<') {
        file.get(lastChar);
        CONSUME_RETURN('=', LE);
        if (lastChar == '|') {
            file.get(lastChar);
            CONSUME_RETURN('=', SHLX_EQ);
            return SHLX;
        }
        if (lastChar == '<') {
            file.get(lastChar);
            CONSUME_RETURN('=', SHL_EQ);
            return SHL;
        }
        return LT;
    }
    if (lastChar == '>') {
        file.get(lastChar);
        CONSUME_RETURN('=', GE);
        CONSUME_RETURN('>', SHR);
        return GT;
    }
    if (lastChar == '=') {
        file.get(lastChar);
        CONSUME_RETURN('=', EQ);
        return ASSIGN;
    }
    CONSUME_RETURN('#', POUND);
    CONSUME_RETURN('@', AT);
    CONSUME_RETURN('$', DOLLAR);
    CONSUME_RETURN('\'', APOSTROPHE);

    // Tokenize Comments
    if (lastChar == ';') {
        file.get(lastChar);
        while (lastChar != EOF && lastChar != '\n' && lastChar != '\r') {
            current_token += lastChar;
            file.get(lastChar);
        }
        return getToken();
    }

    // Identifier: [a-z][a-zA-Z0-9]*
    // Type: [A-Z][a-zA-Z0-9]*
    if (isalpha(lastChar)) {
        Token tok = islower(lastChar) ? IDENTIFIER : TYPE;
        do {
            current_token += lastChar;
            file.get(lastChar);
        } while (isalnum(lastChar));
        return tok;
    }

    // Number: [0-9.]+
    if (isdigit(lastChar) || lastChar == '.') {
        int periods = lastChar == '.';
        do {
            current_token += lastChar;
            file.get(lastChar);
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
        file.get(lastChar);
        while (lastChar != EOF && lastChar != '"') {
            current_token += lastChar;
            file.get(lastChar);
        }
        if (lastChar == '"') {
            file.get(lastChar);
            return STRING;
        } else {
            logError("String literal does not have a closing quote");
            return INVALID;
        }
    }

    // Any other character
    logError("Invalid token");
    return INVALID;
}
