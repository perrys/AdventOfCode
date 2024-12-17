
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

scp::Coordinate update(scp::Grid& grid, char c, scp::Coordinate pos) {
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
    auto pred = [&grid](auto c) { return c == '[' || c == ']' || c == 'O'; };
    assert(pred(target.value()));
    auto next = pos.move(dir);
    while (pred(grid.get(next).value())) {
        next = next.move(dir);
    }
    switch (grid.get(next).value()) {
    case '.':
        // shuffle along - the swap(first,last) shortcut of part 1 doesnt work for part 2
        for (auto temp = next; temp != pos; temp = temp.move(dir, -1)) {
            auto lastVal = grid.get(temp.move(dir, -1)).value();
            grid.set(temp, lastVal);
        }
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

bool recursiveTestPush(const scp::Grid& grid, const scp::Direction dir, const scp::Coordinate pos) {
    assert(dir == scp::NORTH || dir == scp::SOUTH);
    auto target = grid.getWithOffsets(pos, dir);
    if (!target || target.value() == '#') {
        return false;
    }
    if (target.value() == '.') {
        return true;
    }
    assert(target.value() == '[' || target.value() == ']');
    if (target.value() == '[') {
        return recursiveTestPush(grid, dir, pos.move(dir)) &&
               recursiveTestPush(grid, dir, pos.move(dir + scp::EAST));
    }
    if (target.value() == ']') {
        return recursiveTestPush(grid, dir, pos.move(dir)) &&
               recursiveTestPush(grid, dir, pos.move(dir + scp::WEST));
    }
    assert(false);
    return false; // unreachable;
}

void recursiveUpdate(scp::Grid& grid, const scp::Direction dir, scp::Coordinate pos) {
    assert(dir == scp::NORTH || dir == scp::SOUTH);
    auto target = grid.getWithOffsets(pos, dir);
    if (target.value() == '.') {
        grid.set(pos.move(dir), grid.get(pos).value());
        grid.set(pos, '.');
        return;
    }
    assert(target.value() == '[' || target.value() == ']');
    if (target.value() == '[') {
        recursiveUpdate(grid, dir, pos.move(dir));
        recursiveUpdate(grid, dir, pos.move(dir + scp::EAST));
    } else if (target.value() == ']') {
        recursiveUpdate(grid, dir, pos.move(dir));
        recursiveUpdate(grid, dir, pos.move(dir + scp::WEST));
    }
    grid.set(pos.move(dir), grid.get(pos).value());
    grid.set(pos, '.');
}

scp::Coordinate updateWide(scp::Grid& grid, char c, scp::Coordinate pos) {
    auto dir = toDirection(c);
    if (dir == scp::EAST || dir == scp::WEST) {
        // horizontal moves are pretty similar to part 1
        return update(grid, c, pos);
    }
    if (recursiveTestPush(grid, dir, pos)) {
        recursiveUpdate(grid, dir, pos);
        return pos.move(dir);
    }
    return pos;
}

size_t calculate(const scp::Grid& grid) {
    size_t total = 0;
    for (size_t iy = 0; iy < grid.height(); ++iy) {
        for (size_t ix = 0; ix < grid.width(); ++ix) {
            const auto thing = grid.get({ix, iy}).value();
            if (thing == 'O' || thing == '[') {
                total += 100 * iy + ix;
            }
        }
    }
    return total;
}

scp::Grid wideGrid(const std::vector<std::string>& lines) {
    std::vector<std::string> result;
    result.reserve(lines.size());
    for (auto line : lines) {
        std::string row;
        row.reserve(line.size() * 2);
        for (auto tile : line) {
            switch (tile) {
            case '#':
            case '.':
                row.push_back(tile);
                row.push_back(tile);
                break;
            case 'O':
                row.push_back('[');
                row.push_back(']');
                break;
            case '@':
                row.push_back(tile);
                row.push_back('.');
                break;
            default:
                throw std::runtime_error(std::string("unknown tile: ") + tile);
            }
        }
        result.push_back(row);
    }
    return result;
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
    scp::Coordinate startPos;
    for (size_t iy = 0; iy < grid.height(); ++iy) {
        for (size_t ix = 0; ix < grid.width(); ++ix) {
            if (grid.get({ix, iy}) == '@') {
                startPos = {ix, iy};
                goto found;
            }
        }
    }
    std::cerr << "couldn't find robot start point" << std::endl;
    return -1;

found:

    scp::Coordinate position = startPos;
    for (auto moveline : moves) {
        for (auto move : moveline) {
            position = update(grid, move, position);
            // std::cout << "position: " << position << std::endl;
            // grid.print();
        }
    }
    std::cout << "part1 total: " << calculate(grid) << std::endl;

    auto bigGrid = wideGrid(std::vector(lines.begin(), lines.begin() + partition.value()));
    // bigGrid.print();
    position = {startPos.ix * 2, startPos.iy};
    for (auto moveline : moves) {
        for (auto move : moveline) {
            position = updateWide(bigGrid, move, position);
            // std::cout << move << " position: " << position << std::endl;
            // bigGrid.print();
        }
    }
    std::cout << "part2 total: " << calculate(bigGrid) << std::endl;
}
