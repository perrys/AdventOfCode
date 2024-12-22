#include "lib/file_utils.hpp"
#include "lib/grid.hpp"
#include "lib/transform.hpp"

#include <assert.h>

#include <algorithm>
#include <cmath>
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
 * Day 18: RAM Run
 *
 * See <https://adventofcode.com/2024>
 */

template <typename T> std::ostream& operator<<(std::ostream& ostream, const std::vector<T>& vals) {
    bool first = true;
    for (auto i : vals) {
        if (first) {
            first = false;
        } else {
            ostream << ",";
        }
        ostream << (size_t)i;
    }
    return ostream;
}

namespace {

const std::array<scp::Direction, 4> DIRECTIONS{scp::NORTH, scp::SOUTH, scp::WEST, scp::EAST};
std::vector<scp::Coordinate> neighbours(const scp::Grid& grid, scp::Coordinate current) {
    std::vector<scp::Coordinate> result;
    for (auto dir : DIRECTIONS) {
        if (grid.getWithOffsets(current, dir) == '.') {
            result.push_back(current.move(dir));
        }
    }
    return result;
}

void dijkstra(const scp::Grid& grid, scp::Coordinate start, scp::Coordinate end, size_t& minCost) {

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
        for (auto nextStep : neighbours(grid, current)) {
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

    constexpr size_t dimension = 71;
    constexpr size_t count = 1024;
    std::vector<std::string> gridlines;
    for (size_t i = 0; i < dimension; ++i) {
        gridlines.emplace_back(dimension, '.');
    }
    scp::Grid grid(std::move(gridlines));
    auto lines = scp::getLines(arguments[1]);
    size_t idx = 0;
    for (auto line : lines | std::ranges::views::filter([](auto s) { return !s.empty(); })) {
        auto numbers =                                                      //
            line                                                            //
            | std::ranges::views::split(std::string(","))                   //
            | std::ranges::views::filter([](auto s) { return !s.empty(); }) //
            | std::ranges::views::transform(scp::Parser<size_t>());
        assert(std::distance(numbers.begin(), numbers.end()) == 2);
        auto iter = numbers.begin();
        scp::Coordinate xy(*iter++, *iter++);
        grid.set(xy, '#');
        idx += 1;
        if (idx == count) {
            break;
        }
    }

    grid.print();
    size_t minCost;
    dijkstra(grid, {0, 0}, {dimension - 1, dimension - 1}, minCost);
    std::cout << "part1 answer: " << minCost << std::endl;
}
