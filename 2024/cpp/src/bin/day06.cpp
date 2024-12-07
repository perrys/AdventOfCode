
#include "lib/file_utils.hpp"
#include "lib/grid.hpp"
#include "lib/transform.hpp"

#include <assert.h>

#include <algorithm>
#include <filesystem>
#include <iostream>
#include <optional>
#include <ranges>
#include <unordered_set>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 6: Guard Gallivant
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

using namespace scp;

size_t walk(Grid grid, CoOrdinate loc) {
    Direction dir = NORTH;
    std::optional<char> next;
    std::unordered_set<CoOrdinate> visited;
    visited.insert(loc);
    while ((next = grid.getWithOffsets(loc, dir))) {
        switch (next.value()) {
        case '.':
        case '^':
            loc = loc.move(dir);
            visited.insert(loc);
            //            std::cout << loc.ix << "," << loc.iy << std::endl;
            break;
        case '#':
            if (EAST == dir) {
                dir = SOUTH;
            } else if (WEST == dir) {
                dir = NORTH;
            } else if (NORTH == dir) {
                dir = EAST;
            } else if (SOUTH == dir) {
                dir = WEST;
            }
        }
    }
    return visited.size();
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
    size_t ix, iy;
    for (iy = 0; iy < lines.size(); ++iy) {
        const auto& line = lines[iy];
        for (ix = 0; ix < line.size(); ++ix) {
            if ('^' == line[ix]) {
                goto done;
            }
        }
    }
    std::cerr << "ERROR: can't find start point" << std::endl;
done:
    const auto grid = Grid::create(std::move(lines));

    std::cout << "part1 answer: " << walk(grid, {ix, iy}) << std::endl;
}
