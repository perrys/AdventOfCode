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
std::vector<scp::Coordinate>
neighbours(const scp::Grid& grid, scp::Coordinate current,
           std::optional<std::pair<scp::Coordinate, scp::Coordinate>> cheatTile) {
    std::vector<scp::Coordinate> result;
    for (auto dir : DIRECTIONS) {
        const auto next = grid.getWithOffsets(current, dir);
        const bool allowed =
            next.has_value() && ((cheatTile.has_value() && current == cheatTile->first &&
                                  current.move(dir) == cheatTile->second) ||
                                 next.value() != '#');
        if (allowed) {
            result.push_back(current.move(dir));
        }
    }
    return result;
}

using Path = std::unordered_map<scp::Coordinate, scp::Coordinate>;

void trimPath(Path& backlinks, const scp::Coordinate end) {
    Path path;
    auto prev = backlinks.find(end);
    assert(prev != backlinks.end());
    while (prev != backlinks.end()) {
        auto prevTile = (*prev).second;
        path.insert(*prev);
        prev = backlinks.find(prevTile);
    }
    backlinks = path;
}

void dijkstra(const scp::Grid& grid, scp::Coordinate start, scp::Coordinate end, size_t& minCost,
              std::optional<Path*> backlinks,
              std::optional<std::pair<scp::Coordinate, scp::Coordinate>> cheatTile) {

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
        for (auto nextStep : neighbours(grid, current, cheatTile)) {
            size_t newCost = currentCost + 1;
            bool minimal = !costs.contains(nextStep) || newCost < costs[nextStep];
            if (minimal) {
                if (backlinks.has_value()) {
                    backlinks.value()->insert_or_assign(nextStep, current);
                }
                costs.insert_or_assign(nextStep, newCost);
                if (nextStep == end && newCost < minCost) {
                    minCost = newCost;
                } else {
                    queue.push_back(nextStep);
                }
            }
        }
    }
    if (backlinks.has_value()) {
        trimPath(*backlinks.value(), end);
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
    constexpr const size_t threshold = 100;

    const auto grid = scp::Grid(scp::getLines(arguments[1]));
    const auto start = grid.search([](auto c) { return c == 'S'; }).value();
    const auto end = grid.search([](auto c) { return c == 'E'; }).value();
    size_t minCost;
    Path path;
    dijkstra(grid, start, end, minCost, &path, {});
    std::cout << "shortest: " << minCost << std::endl;
    // grid.print(path);

    // just so we can print them in the right order:
    std::vector<scp::Coordinate> directedPath;
    auto prev = path.find(end);
    assert(prev != path.end());
    while (prev != path.end()) {
        auto prevTile = (*prev).second;
        directedPath.push_back((*prev).second);
        prev = path.find(prevTile);
    }
    std::ranges::reverse(directedPath);

    size_t part1Count = 0;
    for (auto tile : directedPath) {
        for (auto dir : DIRECTIONS) {
            if (grid.getWithOffsets(tile, dir) == '#') {
                auto cheatTile = tile.move(dir);
                size_t newCost;
                dijkstra(grid, start, end, newCost, {}, std::make_pair(tile, cheatTile));
                assert(newCost <= minCost);
                if ((minCost - newCost) >= threshold) {
                    part1Count += 1;
                }
            }
        }
    }
    std::cout << "part1 answer: " << part1Count << std::endl;
}