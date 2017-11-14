#ifndef BRAINWORDS_H
#define BRAINWORDS_H

#include <string>
#include <cmath>
#include <algorithm>
#include <vector>

std::string generateBFCode(std::string text) {

    std::string res = "";

    std::vector<unsigned char> base;

    // Create the base cells
    for ( auto c : text )
        base.push_back( (c / 10) * 10 );

    // Remove duplicates
    std::sort( base.begin(), base.end() );
    base.erase( std::unique( base.begin(), base.end() ), base.end() );

    // Write base to the tape
    res += std::string(10, '+') + std::string("[");

    for ( auto b : base )
        res += std::string(">") + std::string(b / 10, '+');

    res += std::string(base.size(), '<') + std::string("-]");

    // Write and print all chars
    int lastPos = 0;
    for ( auto c : text ) {

        unsigned char minDist = std::abs( c - base.front() );
        unsigned char minBase = base.front();
        int minPos = 1;

        for ( int i = 0; i < base.size(); ++i ) {

            if ( std::abs( c - base[i] ) < minDist ) {
                minDist = std::abs( c - base[i]);
                minBase = base[i];
                minPos = i + 1;
            }

        }

        unsigned char op = c < minBase ? '-' : '+';
        unsigned char dir = minPos < lastPos ? '<' : '>';

        res += std::string(std::abs(lastPos - minPos), dir);
        res += std::string(std::abs(c - minBase), op);
        res += std::string(".");

        lastPos = minPos;
        base[minPos - 1] = c;

    }

    return res;

}

#endif
