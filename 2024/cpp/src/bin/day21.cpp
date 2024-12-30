#include "lib/file_utils.hpp"
#include "lib/grid.hpp"

#include <assert.h>

#include <algorithm>
#include <functional>
#include <iostream>
#include <limits>
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

void addNorthSouth(size_t displacement, int direction, MoveList& moves) {
    if (0 == displacement)
        return;
    for (size_t i = 0; i != displacement; ++i) {
        moves.push_back({0, direction});
    }
}
void addEastWest(size_t displacement, int direction, MoveList& moves) {
    if (0 == displacement)
        return;
    for (size_t i = 0; i != displacement; ++i) {
        moves.push_back({direction, 0});
    }
}

void getMoves(const scp::Grid& grid, scp::Coordinate start, scp::Coordinate end, MoveList& moves) {
    const auto vec = start.vector(end);
    const auto unitVector = vec.unitize();
    if (vec.dx > 0 && grid.getWithOffsets(start, {0, vec.dy}) != ' ') {
        // heading west and vertically not into a gap
        addNorthSouth(std::abs(vec.dy), unitVector.dy, moves);
        addEastWest(std::abs(vec.dx), unitVector.dx, moves);
        return;
    }
    if (grid.getWithOffsets(start, {vec.dx, 0}) != ' ') {
        // horizontally not into a gap
        addEastWest(std::abs(vec.dx), unitVector.dx, moves);
        addNorthSouth(std::abs(vec.dy), unitVector.dy, moves);
        return;
    }
    // can never start in the same column as a gap
    addNorthSouth(std::abs(vec.dy), unitVector.dy, moves);
    addEastWest(std::abs(vec.dx), unitVector.dx, moves);
}

std::string toString(MoveList&& path) {
    std::stringstream buffer;
    for (auto m : path) {
        buffer << m;
    };
    buffer << 'A';
    return buffer.str();
}

using Cache = std::unordered_map<std::string, std::string>;

std::string processMoves(size_t depth, const std::vector<const scp::Grid*>& grids,
                         std::vector<Cache> caches, const std::string& line) {

    if (grids.size() == depth) {
        return {line};
    }
    if (caches[depth].contains(line)) {
        return caches[depth][line];
    }
    auto keypad = grids[depth];
    scp::Coordinate start = keypad->search([](auto c) { return c == 'A'; }).value();
    std::string result;
    MoveList moveBuffer;
    for (auto endChar : line) {
        const scp::Coordinate end =
            keypad->search([endChar](auto c) { return c == endChar; }).value();
        moveBuffer.clear();
        getMoves(*keypad, start, end, moveBuffer);
        auto nextLevel = processMoves(depth + 1, grids, caches, toString(std::move(moveBuffer)));
        result += nextLevel;
        start = end;
    }
    // caches[depth][line] = result;
    return result;
}

std::string process(const std::string& firstRobotLine, size_t depth) {
    std::vector<const scp::Grid*> grids{&NUMERIC_KEYPAD};
    for (size_t i = 0; i < depth; ++i) {
        grids.push_back(&KEYPAD);
    }
    std::vector<Cache> caches(grids.size());
    std::string shortest = processMoves(0, grids, caches, firstRobotLine);
    return shortest;
}

std::string part1Moves(const std::string& firstRobotLine) {
    auto result = process(firstRobotLine, 2);
    std::cout << result << std::endl;
    return process(firstRobotLine, 2);
}
std::string part2Moves(const std::string& firstRobotLine) {
    return process(firstRobotLine, 2);
}

size_t getNumber(const std::string& line) {
    size_t result = 0;
    for (auto c : line) {
        if (std::isdigit(c)) {
            result *= 10;
            result += c - '0';
        }
    }
    return result;
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
    size_t part1Total = 0;
    size_t part2Total = 0;
    for (auto line : lines) {
        size_t num = getNumber(line);
        auto part1result = part1Moves(line);
        part1Total += part1result.size() * num;
        auto part2result = part2Moves(line);
        part2Total += part2result.size() * num;
    }
    std::cout << "part1 answer: " << part1Total << std::endl;
    std::cout << "part2 answer: " << part2Total << std::endl;
}

/*
                        0     2           9           A
              <         A  ^  A  >   ^ ^  A   v v v   A
    <  v <    A  > >   ^A <A >A vA <^A A >A <vA A A >^A
  <vA <A A >>^A vA A <^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A

" ^A"
"<v>"

"789"
"456"
"123"
" 0A"


 */
