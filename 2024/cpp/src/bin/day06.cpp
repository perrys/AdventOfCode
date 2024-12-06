
#include "lib/file_utils.hpp"
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

namespace std {
template <> struct hash<std::pair<size_t, size_t>> {
    size_t operator()(const std::pair<size_t, size_t>& p) const {
        std::hash<size_t> hasher;
        return hasher(p.first) ^ hasher(p.second);
    }
};
} // namespace std

namespace {

struct Grid {

    std::vector<std::string> rows;
    size_t rowWidth;

    Grid(std::vector<std::string>&& g) : rows(g), rowWidth(g[0].size()) {
    }

    size_t width() const {
        return this->rowWidth;
    }
    size_t height() const {
        return this->rows.size();
    }

    std::optional<char> get(size_t ix, size_t iy) const {
        if (ix < this->rowWidth && iy < this->rows.size()) {
            return this->rows.at(iy).at(ix);
        }
        return {};
    }

    std::optional<char> getWithOffsets(size_t ix, size_t iy, int dx, int dy) const {
        if (ix == 0 && dx < 0) {
            return {};
        }
        if (iy == 0 && dy < 0) {
            return {};
        }
        return this->get(ix + dx, iy + dy);
    }

    static Grid create(std::vector<std::string>&& lines) {
        size_t width = 0;
        for (size_t i = 0; i < lines.size(); ++i) {
            const auto& line = lines[i];
            if (i > 1) {
                if (width != line.length()) {
                    std::cerr << "ERROR: inconsistent line length at " << i << std::endl;
                    return Grid({});
                }
            } else {
                width = line.length();
            }
        }
        return Grid(std::move(lines));
    }
};

using CoOrdinate = std::pair<size_t, size_t>;

enum dydir { NORTH = -1, SOUTH = 1 };
enum dxdir { EAST = 1, WEST = -1 };

size_t walk(Grid grid, size_t ix, size_t iy) {
    int dx = 0;
    int dy = -1;
    std::optional<char> next;
    std::unordered_set<CoOrdinate> visited{{ix, iy}};
    while ((next = grid.getWithOffsets(ix, iy, dx, dy))) {
        switch (next.value()) {
        case '.':
        case '^':
            ix += dx;
            iy += dy;
            visited.insert({ix, iy});
            std::cout << ix << "," << iy << std::endl;
            break;
        case '#':
            if (dxdir::EAST == dx) {
                dx = 0;
                dy = dydir::SOUTH;
            } else if (dxdir::WEST == dx) {
                dx = 0;
                dy = dydir::NORTH;
            } else if (dydir::NORTH == dy) {
                dx = dxdir::EAST;
                dy = 0;
            } else if (dydir::SOUTH == dy) {
                dx = dxdir::WEST;
                dy = 0;
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
done:
    const auto grid = Grid::create(std::move(lines));

    std::cout << "part1 answer: " << walk(grid, ix, iy) << std::endl;
}
