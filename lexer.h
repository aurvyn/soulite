#include <string>
#include "globals.h"

static int gettok() {
    static int lastChar = ' ';
    
    // Skip whitespace
    while (isspace(lastChar)) {
        lastChar = getchar();
    }

    if (lastChar == ',') lastChar = VAR;
    if (lastChar == '\'') lastChar = CONST;
    if (lastChar == '.') lastChar = FUNCTION;
    if (lastChar == '@') lastChar = STRUCT;
    if (lastChar == '#') lastChar = TRAIT;
    if (lastChar == '$') lastChar = IMPORT;

    // Identifier: [a-zA-Z][a-zA-Z0-9]*
    if (isalpha(lastChar)) {
        str = lastChar;
        while (isalnum((lastChar = getchar()))) {
            str += lastChar;
        }
        return IDENTIFIER;
    }

    // Number: [0-9.]+
    if (isdigit(lastChar) || lastChar == '.') {
        int periods = lastChar == '.';
        std::string numStr;
        do {
            numStr += lastChar;
            lastChar = getchar();
            periods += lastChar == '.';
        } while (isdigit(lastChar) || lastChar == '.');
        if (periods > 1) {
            fprintf(stderr, "Error: Invalid number format `%s`\n", numStr.c_str());
            return 0;
        } else if (periods == 1) {
            f_num = std::stof(numStr);
            return FLOAT;
        } else {
            i_num = std::stoi(numStr);
            return INT;
        }
    }

    // Comment or EOF
    if (lastChar == ';') {
        do {
            lastChar = getchar();
        } while (lastChar != EOF && lastChar != '\n' && lastChar != '\r');
        if (lastChar != EOF) return gettok();
    }

    // End of file
    if (lastChar == EOF) return EoF;

    // Any other character
    int thisChar = lastChar;
    lastChar = getchar();
    return thisChar;
}
