
#include "lib/file_utils.hpp"
#include "lib/grid.hpp"
#include "lib/transform.hpp"

#include <assert.h>

#include <algorithm>
#include <iostream>
#include <optional>
#include <ranges>
#include <unordered_map>
#include <unordered_set>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 10: Hoof It
 *
 * See <https://adventofcode.com/2024>
 */

namespace {
void getValidNeighbours(const scp::Grid& grid, scp::Coordinate current,
                        std::vector<scp::Coordinate>& out) {
    std::array<scp::Direction, 4> dirs{scp::NORTH, scp::SOUTH, scp::EAST, scp::WEST};
    out.clear();
    const char value = grid.get(current).value();
    for (auto dir : dirs) {
        if (auto candidate = grid.getWithOffsets(current, dir)) {
            if (1 == candidate.value() - value) {
                out.push_back(current.move(dir));
            }
        }
    }
}

size_t depthFirstSearch(const scp::Grid& grid, scp::Coordinate start) {
    std::vector<scp::Coordinate> queue{start};
    std::vector<scp::Coordinate> validNeighbours;
    std::unordered_set<scp::Coordinate> endPoints;
    std::unordered_set<scp::Coordinate> visited;
    while (!queue.empty()) {
        scp::Coordinate current = queue.back();
        visited.insert(current);
        queue.pop_back();
        if ('9' == grid.get(current).value()) {
            endPoints.insert(current);
            continue;
        }
        getValidNeighbours(grid, current, validNeighbours);
        for (const auto n : validNeighbours) {
            if (!visited.contains(n)) {
                queue.push_back(n);
            }
        }
    }
    return endPoints.size();
}

} // namespace

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }

    auto grid = scp::Grid::create(scp::getLines(arguments[1]));

    size_t total = 0;
    for (size_t iy = 0; iy < grid.height(); ++iy) {
        for (size_t ix = 0; ix < grid.width(); ++ix) {
            if (grid.get({ix, iy}).value() == '0') {
                const size_t score = depthFirstSearch(grid, {ix, iy});
                total += score;
            }
        }
    }
    std::cout << "part1 answer: " << total << std::endl;
}
