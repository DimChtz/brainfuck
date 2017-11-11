#ifndef BRAINWORDS_H
#define BRAINWORDS_H

#include <string>
#include <cmath>

std::string generateBFCode(std::string str) {

    std::string res = "";

    char lastChar = 0;

    for ( auto c : str ) {

        if (c > lastChar) {
            unsigned char factor = sqrt(c - lastChar);
            unsigned char remain = (c - lastChar) - factor * factor;

            res += std::string(factor, '+') + std::string("[>") + std::string(factor, '+') + std::string("<-]") + std::string(">") + std::string(remain, '+') + std::string(".<");
        } else {
            unsigned char factor = sqrt(lastChar - c);
            unsigned char remain = (lastChar - c) - factor * factor;

            res += std::string(factor, '+') + std::string("[>") + std::string(factor, '-') + std::string("<-]") + std::string(">") + std::string(remain, '-') + std::string(".<");
        }

        lastChar = c;

    }

    return res;

}

#endif
