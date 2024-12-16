
#include "lib/file_utils.hpp"
#include "lib/grid.hpp"

#include <assert.h>

#include <algorithm>
#include <cmath>
#include <functional>
#include <iostream>
#include <numeric>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 15: Warehouse Woes
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

scp::Direction toDirection(const char c) {
    switch (c) {
    case '^':
        return scp::NORTH;
    case 'v':
        return scp::SOUTH;
    case '>':
        return scp::EAST;
    case '<':
        return scp::WEST;
    default:

        throw std::runtime_error(std::string("unrecognized move: ") + c);
    }
}

scp::Coordinate update(scp::Coordinate pos, scp::Grid& grid, char c) {
    auto dir = toDirection(c);
    auto target = grid.getWithOffsets(pos, dir);

    if (!target || target.value() == '#') {
        return pos;
    }
    if (target.value() == '.') {
        grid.set(pos, '.');
        pos = pos.move(dir);
        grid.set(pos, '@');
        return pos;
    }
    assert(target.value() == 'O');
    auto next = pos.move(dir);
    while (grid.get(next) == 'O') {
        next = next.move(dir);
    }
    switch (grid.get(next).value()) {
    case '.':
        grid.set(next, 'O');
        grid.set(pos, '.');
        pos = pos.move(dir);
        grid.set(pos, '@');
        break;
    case '#':
        break;
    default:
        throw std::runtime_error(std::string("unrecognized tile: ") + c);
    }
    return pos;
}

size_t calculate(const scp::Grid& grid) {
    size_t total = 0;
    for (size_t iy = 0; iy < grid.height(); ++iy) {
        for (size_t ix = 0; ix < grid.width(); ++ix) {
            if (grid.get({ix, iy}) == 'O') {
                total += 100 * iy + ix;
            }
        }
    }
    return total;
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
    std::optional<size_t> partition;
    for (size_t idx = 0; idx < lines.size(); ++idx) {
        if (lines[idx].empty()) {
            partition = idx;
            break;
        }
    }
    assert(partition.has_value());

    const auto moves = std::vector(lines.begin() + partition.value() + 1, lines.end());
    auto grid = scp::Grid(std::vector(lines.begin(), lines.begin() + partition.value()));
    scp::Coordinate position;
    for (size_t iy = 0; iy < grid.height(); ++iy) {
        for (size_t ix = 0; ix < grid.width(); ++ix) {
            if (grid.get({ix, iy}) == '@') {
                position = {ix, iy};
                goto found;
            }
        }
    }
    std::cerr << "couldn't find robot start point" << std::endl;
    return -1;

found:

    for (auto moveline : moves) {
        for (auto move : moveline) {
            position = update(position, grid, move);
            // std::cout << "position: " << position << std::endl;
            // grid.print();
        }
    }
    std::cout << "part1 total: " << calculate(grid) << std::endl;
}
