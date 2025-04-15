#pragma once
#include <string>

static std::string str;
static float f_num;
static int i_num;
static bool boolean;

enum Token {
    EoF = -1,
    VAR = -2,
    CONST = -3,
    FUNCTION = -4,
    STRUCT = -5,
    TRAIT = -6,
    IMPORT = -7,
    IDENTIFIER = -8,
    FLOAT = -9,
    INT = -10,
};