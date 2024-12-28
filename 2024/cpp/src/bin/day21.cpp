#include "lib/file_utils.hpp"
#include "lib/grid.hpp"

#include <assert.h>

#include <algorithm>
#include <functional>
#include <iostream>
#include <ranges>
#include <sstream>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 21: Keypad Conundrum
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

const scp::Grid NUMERIC_KEYPAD(std::vector<std::string>{"789", //
                                                        "456", //
                                                        "123", //
                                                        " 0A"});

const scp::Grid KEYPAD(std::vector<std::string>{" ^A", //
                                                "<v>"});

using MoveList = std::vector<scp::Direction>;

void addNorthSouth(int64_t displacement, MoveList& moves) {
    if (0 == displacement)
        return;
    int unit = displacement / std::abs(displacement);
    for (int64_t i = 0; i != displacement; i += unit) {
        moves.push_back({0, unit});
    }
}
void addEastWest(int64_t displacement, MoveList& moves) {
    if (0 == displacement)
        return;
    int unit = displacement / std::abs(displacement);
    for (int64_t i = 0; i != displacement; i += unit) {
        moves.push_back({unit, 0});
    }
}

void getMoves(const scp::Grid& grid, scp::Coordinate start, scp::Coordinate end, MoveList& moves) {
    const auto [dx, dy] = start.displacement(end);
    if (grid.get({0, 0}) == ' ') { // space is at the top
        if (dy > 0) {              // SOUTH
            addNorthSouth(dy, moves);
            addEastWest(dx, moves);
        } else {
            addEastWest(dx, moves);
            addNorthSouth(dy, moves);
        }
    } else {          // space is at the bottom
        if (dy > 0) { // SOUTH
            addEastWest(dx, moves);
            addNorthSouth(dy, moves);
        } else {
            addNorthSouth(dy, moves);
            addEastWest(dx, moves);
        }
    }
}

std::string part1Moves(const std::string& line) {

    MoveList moveBuffer;

    auto processLine = [&moveBuffer](auto line, auto keypad) {
        scp::Coordinate start = keypad.search([](auto c) { return c == 'A'; }).value();
        std::stringstream result;
        for (auto ch : line) {
            auto end = keypad.search([ch](auto c) { return c == ch; }).value();
            moveBuffer.clear();
            getMoves(keypad, start, end, moveBuffer);
            for (auto m : moveBuffer) {
                result << m;
            };
            result << 'A';
            start = end;
        }
        return result.str();
    };

    const std::string firstLine = processLine(line, NUMERIC_KEYPAD);
    const std::string secondLine = processLine(firstLine, KEYPAD);
    const std::string thirdLine = processLine(secondLine, KEYPAD);
    // std::cout << line << std::endl;
    // std::cout << firstLine << std::endl;
    // std::cout << secondLine << std::endl;
    // std::cout << thirdLine << std::endl;
    return thirdLine;
}

} // namespace

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }
    auto lines = scp::getLines(arguments[1]);
    for (auto line : lines) {
        auto result = part1Moves(line);
        std::cout << line << ": " << result << std::endl;
    }
}
