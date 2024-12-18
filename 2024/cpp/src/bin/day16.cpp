
#include "lib/file_utils.hpp"
#include "lib/grid.hpp"
#include "lib/hash_utils.hpp"

#include <assert.h>
#include <bits/ranges_algo.h>

#include <algorithm>
#include <cmath>
#include <functional>
#include <iostream>
#include <limits>
#include <numeric>
#include <unordered_map>
#include <unordered_set>
#include <utility>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 16: Reindeer Maze
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

struct PathStep { // a Ray
    scp::Coordinate loc;
    scp::Direction dir;
    std::optional<std::pair<scp::Coordinate, scp::Direction>> prev;

    bool operator==(const PathStep& other) const {
        return this->loc == other.loc && this->dir == other.dir;
    }
};
} // namespace

namespace std {
template <> struct hash<PathStep> {
    size_t operator()(const PathStep& p) const {
        std::hash<scp::Coordinate> hash1;
        std::hash<scp::Direction> hash2;
        return hash1(p.loc) ^ hash2(p.dir);
    }
};
} // namespace std

namespace {

const std::array<scp::Direction, 4> DIRECTIONS{scp::NORTH, scp::SOUTH, scp::WEST, scp::EAST};

std::vector<PathStep> neighbours(const scp::Grid& grid, PathStep current) {
    std::vector<PathStep> result;
    for (auto dir : DIRECTIONS) {
        if (dir != current.dir.opposite() && grid.getWithOffsets(current.loc, dir) != '#') {
            result.push_back(
                {current.loc.move(dir), dir, std::make_pair(current.loc, current.dir)});
        }
    }
    return result;
}

size_t directionCost(scp::Direction current, scp::Direction next) {
    const size_t rotationCost = 1000;
    return (current == next) ? 0 : rotationCost;
}

[[maybe_unused]] std::vector<PathStep> reconstructPath(std::unordered_map<PathStep, size_t> costs,
                                                       PathStep end) {
    std::vector<PathStep> result;
    PathStep step = end;
    while (true) {
        result.push_back(step);
        if (!step.prev) {
            break;
        }
        PathStep test{step.prev->first, step.prev->second, {}};
        auto iter = costs.find(test);
        assert(iter != costs.end());
        step = (*iter).first;
    }
    std::ranges::reverse(result);
    return result;
}

size_t dijkstra(const scp::Grid& grid, PathStep start, scp::Coordinate end) {

    std::vector<PathStep> queue{start};
    std::unordered_map<PathStep, size_t> costs{{start, 0}};

    auto sorter = [&costs](PathStep lhs, PathStep rhs) {
        assert(costs.contains(lhs));
        assert(costs.contains(rhs));
        return costs[rhs] < costs[lhs]; // reverse order
    };

    size_t minCost = std::numeric_limits<size_t>::max();
    while (!queue.empty()) {
        std::ranges::sort(queue.begin(), queue.end(), sorter);
        const auto current = queue.back();
        queue.pop_back();
        assert(costs.contains(current));
        const size_t currentCost = costs[current];
        for (auto nextStep : neighbours(grid, current)) {
            size_t newCost = currentCost + directionCost(current.dir, nextStep.dir) + 1;
            bool minimal = !costs.contains(nextStep) || newCost < costs[nextStep];
            if (minimal) {
                queue.push_back(nextStep);
                costs.insert_or_assign(nextStep, newCost);
                if (nextStep.loc == end && newCost < minCost) {
                    minCost = newCost;
                }
            }
        }
    }
    return minCost;
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
    std::optional<scp::Coordinate> start = grid.search([](auto c) { return c == 'S'; });
    std::optional<scp::Coordinate> end = grid.search([](auto c) { return c == 'E'; });
    assert(start.has_value() && end.has_value());

    const auto cost = dijkstra(grid, {start.value(), scp::EAST, {}}, end.value());
    std::cout << "part1 answer = " << cost << std::endl;
}
