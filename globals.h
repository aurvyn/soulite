#pragma once
#include <cstdio>

enum Token {
    RANGE = 0,  // ..
    AND = 10,   // &&
    OR = 11,    // ||
    AMP = 20,   // &
    PIPE = 21,  // |
    LT = 30,    // <
    GT = 31,    // >
    LE = 32,    // <=
    GE = 33,    // >=
    EQ = 34,    // ==
    NE = 35,    // !=
    SHL = 40,   // <<
    SHR = 41,   // >>
    SHLX = 42,  // <|
    SHRX = 43,  // |>
    PLUS = 50,  // +
    DASH = 51,  // -
    ASTER = 60, // *
    DIV = 61,   // /
    MOD = 62,   // %
    EXP = 70,   // **
    DOT = 80,   // .
    INVALID,
    EoF,
    EXCL,       // !
    LPAREN,     // (
    RPAREN,     // )
    LBRACE,     // {
    RBRACE,     // }
    LBRACK,     // [
    RBRACK,     // ]
    COMMA,      // ,
    COLON,      // :
    TILDE,      // ~
    INC,        // ++
    DEC,        // --
    XOR,        // ^
    POUND,      // #
    AT,         // @
    DOLLAR,     // $
    APOSTROPHE, // '
    ARROW,      // ->
    ASSIGN,     // =
    PLUS_EQ,    // +=
    MINUS_EQ,   // -=
    MUL_EQ,     // *=
    DIV_EQ,     // /=
    MOD_EQ,     // %=
    AND_EQ,     // &=
    OR_EQ,      // |=
    INV_EQ,     // ~=
    XOR_EQ,     // ^=
    EXP_EQ,     // **=
    SHL_EQ,     // <<=
    SHLX_EQ,    // <|=
    IDENTIFIER, // [a-z][a-zA-Z0-9]*
    TYPE,       // [A-Z][a-zA-Z0-9]*
    COMMENT,    // ;
    FLOAT,      // [0-9]*.[0-9]+
    INT,        // [0-9]+
    STRING,     // ".*"
};

static void logError(const char *str) {
    fprintf(stderr, "Error: %s\n", str);
}
