
#include "lib/file_utils.hpp"
#include "lib/grid.hpp"

#include <assert.h>

#include <algorithm>
#include <cmath>
#include <functional>
#include <iostream>
#include <numeric>
#include <unordered_map>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 16: Reindeer Maze
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

struct Path {
    scp::Coordinate current;
    size_t cost;
};

void dijkstra(const Grid& grid, scp::Coordinate start, scp::Coordinate end) {
    std::vector<scp::Coordinate> queue{start};
    std::unordered_map<scp::Coordinate, size_t> costs{{start, 0}};
    auto sorter =
        [&costs](scp::Coordinate lhs, scp::Coordinate rhs) {
            assert(costs.contains(lhs));
            assert(costs.contains(rhs));
            return costs[rhs] < costs[lhs];
        }

    while (!queue.empty()) {
        queue.sort(queue.begin(), queue.end(), sorter);
        auto next = queue.back();
        queue.pop_back();
        for
    }
}

} // namespace

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }
    scp::Grid grid(scp::getLines(arguments[1]));
}
