
#include "lib/file_utils.hpp"
#include "lib/grid.hpp"
#include "lib/transform.hpp"

#include <algorithm>
#include <filesystem>
#include <iostream>
#include <optional>
#include <ranges>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 4: Ceres Search
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

using namespace scp;

size_t subsearch(const Grid& grid, CoOrdinate c, Direction d) {
    std::array<char, 3> letters = {'M', 'A', 'S'};
    for (auto letter : letters) {
        auto opt = grid.getWithOffsets(c, d);
        if (!opt || opt.value() != letter) {
            return 0;
        }
        c = c.move(d);
    }
    return 1;
}

size_t part1Search(const Grid& grid, CoOrdinate c) {
    size_t total = 0;
    total += subsearch(grid, c, NORTH);
    total += subsearch(grid, c, SOUTH);
    total += subsearch(grid, c, EAST);
    total += subsearch(grid, c, WEST);
    total += subsearch(grid, c, NORTH + EAST);
    total += subsearch(grid, c, NORTH + WEST);
    total += subsearch(grid, c, SOUTH + EAST);
    total += subsearch(grid, c, SOUTH + WEST);
    return total;
}

size_t part2Search(const Grid& grid, CoOrdinate c) {
    auto nw = grid.getWithOffsets(c, NORTH + WEST);
    auto ne = grid.getWithOffsets(c, NORTH + EAST);
    auto sw = grid.getWithOffsets(c, SOUTH + WEST);
    auto se = grid.getWithOffsets(c, SOUTH + EAST);
    if (!nw || !ne || !sw || !se) {
        return 0;
    }
    auto line1 = std::vector{nw.value(), se.value()};
    auto line2 = std::vector{ne.value(), sw.value()};
    std::ranges::sort(line1);
    std::ranges::sort(line2);
    const auto expected = std::vector{'M', 'S'};
    if (line1 != expected || line2 != expected) {
        return 0;
    }
    return 1;
}

} // namespace

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }

    const auto grid = Grid::create(scp::getLines(arguments[1]));
    int part1Total = 0;
    int part2Total = 0;

    for (size_t iy = 0; iy < grid.height(); ++iy) {
        for (size_t ix = 0; ix < grid.width(); ++ix) {
            auto letter = grid.get({ix, iy}).value();
            if ('X' == letter) {
                part1Total += part1Search(grid, {ix, iy});
            }
            if ('A' == letter) {
                part2Total += part2Search(grid, {ix, iy});
            }
        }
    }
    std::cout << "part1 answer: " << part1Total << std::endl;
    std::cout << "part2 answer: " << part2Total << std::endl;
}
