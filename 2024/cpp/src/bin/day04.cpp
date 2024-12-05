
#include "lib/file_utils.hpp"
#include "lib/transform.hpp"

#include <algorithm>
#include <filesystem>
#include <iostream>
#include <optional>
#include <ranges>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 4: Ceres Search
 *
 * See <https://adventofcode.com/2024>
 */

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

    static Grid create(std::filesystem::path input) {
        auto lines = scp::getLines(input);
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

size_t subsearch(const Grid& grid, size_t ix, size_t iy, int dx, int dy) {
    std::array<char, 3> letters = {'M', 'A', 'S'};
    for (auto letter : letters) {
        auto opt = grid.getWithOffsets(ix, iy, dx, dy);
        if (!opt || opt.value() != letter) {
            return 0;
        }
        ix += dx;
        iy += dy;
    }
    return 1;
}

size_t part1Search(const Grid& grid, size_t ix, size_t iy) {
    size_t total = 0;
    total += subsearch(grid, ix, iy, 0, 1);
    total += subsearch(grid, ix, iy, 0, -1);
    total += subsearch(grid, ix, iy, 1, 1);
    total += subsearch(grid, ix, iy, 1, -1);
    total += subsearch(grid, ix, iy, -1, 1);
    total += subsearch(grid, ix, iy, -1, -1);
    total += subsearch(grid, ix, iy, 1, 0);
    total += subsearch(grid, ix, iy, -1, 0);
    return total;
}

size_t part2Search(const Grid& grid, size_t ix, size_t iy) {
    auto topLeft = grid.getWithOffsets(ix, iy, -1, -1);
    auto topRight = grid.getWithOffsets(ix, iy, 1, -1);
    auto bottomLeft = grid.getWithOffsets(ix, iy, -1, 1);
    auto bottomRight = grid.getWithOffsets(ix, iy, 1, 1);
    if (!topLeft || !topRight || !bottomLeft || !bottomRight) {
        return 0;
    }
    auto line1 = std::vector{topLeft.value(), bottomRight.value()};
    auto line2 = std::vector{topRight.value(), bottomLeft.value()};
    std::ranges::sort(line1);
    std::ranges::sort(line2);
    const auto expected = std::vector{'M', 'S'};
    if (line1 != expected || line2 != expected) {
        return 0;
    }
    return 1;
}

} // namespace

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }

    const auto grid = Grid::create(arguments[1]);
    int part1Total = 0;
    int part2Total = 0;

    for (size_t iy = 0; iy < grid.height(); ++iy) {
        for (size_t ix = 0; ix < grid.width(); ++ix) {
            auto letter = grid.get(ix, iy).value();
            if ('X' == letter) {
                part1Total += part1Search(grid, ix, iy);
            }
            if ('A' == letter) {
                part2Total += part2Search(grid, ix, iy);
            }
        }
    }
    std::cout << "part1 answer: " << part1Total << std::endl;
    std::cout << "part2 answer: " << part2Total << std::endl;
}
