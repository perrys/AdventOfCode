
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
 * Day 4
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
};

size_t subsearch(const Grid& grid, size_t ix, size_t iy, int dx, int dy) {
    std::array<char, 3> letters = {'M', 'A', 'S'};
    for (auto letter : letters) {
        if (ix == 0 && dx < 0) {
            return false;
        }
        if (iy == 0 && dy < 0) {
            return false;
        }
        ix += dx;
        iy += dy;
        auto opt = grid.get(ix, iy);
        if (!opt || opt.value() != letter) {
            return false;
        }
    }
    return true;
}

size_t search(const Grid& grid, size_t ix, size_t iy) {
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

Grid parseGrid(std::filesystem::path input) {
    auto lines = scp::getLines(input);
    size_t width = 0;
    for (size_t i = 0; i < lines.size(); ++i) {
        const auto& line = lines[i];
        if (i > 1) {
            if (width != line.length()) {
                std::cerr << "inconsistent line length at " << i << std::endl;
                return Grid({});
            }
        } else {
            width = line.length();
        }
    }
    return Grid(std::move(lines));
}

} // namespace

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }

    const auto grid = parseGrid(arguments[1]);
    int total = 0;
    for (size_t iy = 0; iy < grid.height(); ++iy) {
        for (size_t ix = 0; ix < grid.width(); ++ix) {
            auto letter = grid.get(ix, iy).value();
            if ('X' == letter) {
                total += search(grid, ix, iy);
            }
        }
    }
    std::cout << "part1 answer: " << total << std::endl;
    return 0;
}
