#include "lib/file_utils.hpp"
#include "lib/grid.hpp"
#include "lib/transform.hpp"

#include <assert.h>

#include <algorithm>
#include <cstdint>
#include <functional>
#include <iostream>
#include <limits>
#include <numeric>
#include <ranges>
#include <unordered_map>
#include <unordered_set>
#include <utility>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 20: Race Condition
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

const std::array<scp::Direction, 4> DIRECTIONS{scp::NORTH, scp::SOUTH, scp::WEST, scp::EAST};
std::vector<scp::Coordinate> neighbours(const scp::Grid& grid, scp::Coordinate current,
                                        bool cheat = false) {
    std::vector<scp::Coordinate> result;
    for (auto dir : DIRECTIONS) {
        const auto next = grid.getWithOffsets(current, dir);
        const bool allowed = cheat ? next.has_value() : (next.has_value() && next.value() != '#');
        if (allowed) {
            result.push_back(current.move(dir));
        }
    }
    return result;
}

void dijkstra(const scp::Grid& grid, scp::Coordinate start, scp::Coordinate end, size_t& minCost,
              std::optional<size_t> cheatStart) {

    std::vector<scp::Coordinate> queue{start};
    std::unordered_map<scp::Coordinate, size_t> costs{{start, 0}};

    auto sorter = [&costs](scp::Coordinate lhs, scp::Coordinate rhs) {
        assert(costs.contains(lhs));
        assert(costs.contains(rhs));
        return costs[rhs] < costs[lhs]; // reverse order
    };

    minCost = std::numeric_limits<size_t>::max();
    while (!queue.empty()) {
        std::ranges::sort(queue.begin(), queue.end(), sorter);
        const auto current = queue.back();
        queue.pop_back();
        assert(costs.contains(current));
        const size_t currentCost = costs[current];
        const bool cheat = currentCost == cheatStart;
        for (auto nextStep : neighbours(grid, current, cheat)) {
            // TODO - if there is more than one cheat path we have to measure both somehow
            size_t newCost = currentCost + 1;
            bool minimal = !costs.contains(nextStep) || newCost < costs[nextStep];
            if (minimal) {
                costs.insert_or_assign(nextStep, newCost);
                if (nextStep == end && newCost < minCost) {
                    minCost = newCost;
                } else {
                    queue.push_back(nextStep);
                }
            }
        }
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

    const auto grid = scp::Grid(scp::getLines(arguments[1]));
    const auto start = grid.search([](auto c) { return c == 'S'; }).value();
    const auto end = grid.search([](auto c) { return c == 'E'; }).value();
    size_t minCost;
    dijkstra(grid, start, end, minCost, {});
    std::cout << "shortest: " << minCost << std::endl;
    size_t part1Count = 0;
    for (size_t i = 0; i < minCost; ++i) {
        size_t newCost;
        dijkstra(grid, start, end, newCost, i);
        std::cout << i << ": " << minCost - newCost << std::endl;
        assert(newCost <= minCost);
        if ((minCost - newCost) >= 100) {
            part1Count += 1;
        }
    }
    std::cout << "part1 answer: " << part1Count << std::endl;
}
