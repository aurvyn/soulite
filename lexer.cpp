#include <string>

enum Token {
    EoF = -1,
    VAR = -2,
    CONST = -3,
    FUNCTION = -4,
    STRUCT = -5,
    TRAIT = -6,
    IMPORT = -7,
    IDENTIFIER = -8,
    NUMBER = -9,
};

static std::string IdentifierStr;
static double NumVal;

static int gettok() {
    static int LastChar = ' ';
    
    // Skip whitespace
    while (isspace(LastChar)) {
        LastChar = getchar();
    }

    if (LastChar == ',') return VAR;
    if (LastChar == '\'') return CONST;
    if (LastChar == '.') return FUNCTION;
    if (LastChar == '@') return STRUCT;
    if (LastChar == '#') return TRAIT;
    if (LastChar == '$') return IMPORT;

    // Identifier: [a-zA-Z][a-zA-Z0-9]*
    if (isalpha(LastChar)) {
        IdentifierStr = LastChar;
        while (isalnum((LastChar = getchar()))) {
            IdentifierStr += LastChar;
        }
        return IDENTIFIER;
    }

    // Number: [0-9.]+
    if (isdigit(LastChar) || LastChar == '.') {
        int periods = LastChar == '.';
        std::string NumStr;
        do {
            NumStr += LastChar;
            LastChar = getchar();
            periods += LastChar == '.';
        } while (isdigit(LastChar) || LastChar == '.');
        if (periods > 1) {
            fprintf(stderr, "Error: Invalid number format `%s`\n", NumStr.c_str());
            return 0;
        }
        NumVal = strtod(NumStr.c_str(), 0);
        return NUMBER;
    }

    // Comment or EOF
    if (LastChar == ';') {
        do {
            LastChar = getchar();
        } while (LastChar != EOF && LastChar != '\n' && LastChar != '\r');
        if (LastChar != EOF) return gettok();
    }

    // End of file
    if (LastChar == EOF) return EoF;

    // Any other character
    int ThisChar = LastChar;
    LastChar = getchar();
    return ThisChar;
}
