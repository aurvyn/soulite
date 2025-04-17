#include <iostream>
#include <fstream>
#include "parser.h"

int main() {
    file = std::ifstream("test/expr.soul");

    if (!file) {
        std::cerr << "Error opening file" << std::endl;
        return 1;
    }

    mainLoop();
    file.close();
    return 0;
}