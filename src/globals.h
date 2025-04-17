#pragma once
#include <cstdio>

enum Token {
    ASSIGN = 0, // =
    RANGE = 10, // ..
    AND = 20,   // &&
    OR = 21,    // ||
    AMP = 30,   // &
    PIPE = 31,  // |
    LT = 40,    // <
    GT = 41,    // >
    LE = 42,    // <=
    GE = 43,    // >=
    EQ = 44,    // ==
    NE = 45,    // !=
    SHL = 50,   // <<
    SHR = 51,   // >>
    SHLX = 52,  // <|
    SHRX = 53,  // |>
    PLUS = 60,  // +
    DASH = 61,  // -
    ASTER = 70, // *
    DIV = 71,   // /
    MOD = 72,   // %
    EXP = 80,   // **
    DOT = 90,   // .
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
