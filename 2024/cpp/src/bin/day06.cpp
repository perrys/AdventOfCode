
#include "lib/file_utils.hpp"
#include "lib/grid.hpp"
#include "lib/hash_utils.hpp"
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

std::optional<size_t> walk(Grid grid, Coordinate loc,
                           std::vector<std::pair<Coordinate, Direction>>* recorder = nullptr) {
    Direction dir = NORTH;
    std::optional<char> next;
    std::unordered_set<Coordinate> visited;
    std::unordered_set<std::pair<Coordinate, Direction>> rays;
    visited.insert(loc);
    if (recorder) {
        recorder->emplace_back(loc, dir);
    }
    while ((next = grid.getWithOffsets(loc, dir))) {
        auto ray = std::make_pair(loc, dir);
        switch (next.value()) {
        case '.':
        case '^':
            loc = loc.move(dir);
            visited.insert(loc);
            if (rays.contains(ray)) {
                return {}; // circular path
            }
            rays.insert(ray);
            if (recorder) {
                recorder->push_back(ray);
            }
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
    return -1;

done:
    const scp::Coordinate startPoint{ix, iy};
    auto grid = Grid::create(std::move(lines));
    std::vector<std::pair<scp::Coordinate, scp::Direction>> originalPath;
    std::cout << "part1 answer: " << walk(grid, startPoint, &originalPath).value() << std::endl;

    std::unordered_set<Coordinate> blocks;
    for (const auto& point : originalPath) {
        auto [loc, dir] = point;
        Coordinate block = loc.move(dir);
        auto opt = grid.get(block);
        if (opt && opt.value() != '#') {
            grid.set(block, '#');
            if (!walk(grid, startPoint)) {
                //std::cout << "circular path at " << block << std::endl;
                blocks.insert(block);
            }
            grid.set(block, opt.value());
        }
    }
    std::cout << "part2 answer: " << blocks.size() << std::endl;
}
