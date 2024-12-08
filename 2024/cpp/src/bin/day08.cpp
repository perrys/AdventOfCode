
#include "lib/file_utils.hpp"
#include "lib/grid.hpp"
#include "lib/transform.hpp"

#include <assert.h>

#include <algorithm>
#include <iostream>
#include <optional>
#include <ranges>
#include <unordered_map>
#include <unordered_set>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 8: Resonant Collinearity
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

const auto npos = std::string::npos;

using Coord = scp::GenCoordinate<int>;
using CoordList = std::vector<Coord>;

void addAntinodes(Coord a, Coord b, CoordList& result) {
    auto [dx, dy] = a.displacement(b);
    result.emplace_back(a.ix + 2 * dx, a.iy + 2 * dy);
    result.emplace_back(b.ix - 2 * dx, b.iy - 2 * dy);
}

void calculateNodeLocations(const CoordList::const_iterator begin,
                            const CoordList::const_iterator end, CoordList& result) {
    if (begin == end) {
        return;
    }
    const Coord myloc = *begin;
    for (auto iter = begin + 1; iter != end; ++iter) {
        addAntinodes(myloc, (*iter), result);
    }
    calculateNodeLocations(begin + 1, end, result);
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
    size_t width = 0;
    std::unordered_map<char, CoordList> map;

    for (uint32_t iy = 0; iy < lines.size(); ++iy) {
        const auto line = lines[iy];
        if (0 == iy) {
            width = line.size();
        } else {
            assert(line.size() == width); // irregular grid
        }
        for (uint32_t ix = 0; ix < line.size(); ++ix) {
            const char c = line[ix];
            switch (c) {
            case '.':
                break;
            default:
                auto [iter, _flag] = map.try_emplace(c);
                (*iter).second.push_back(Coord{(int)ix, (int)iy});
            }
        }
    }

    std::unordered_set<Coord> uniqueLocations;
    for (auto frequency : map) {
        const auto& [_freq, locs] = frequency;
        CoordList antinodeLocations;
        calculateNodeLocations(locs.begin(), locs.end(), antinodeLocations);
        auto iter = antinodeLocations |
                    std::ranges::views::filter([width, height = lines.size()](const Coord c) {
                        return c.ix >= 0 && c.ix < (int)width && c.iy >= 0 && c.iy < (int)height;
                    });
        for (auto loc : iter) {
            uniqueLocations.insert(loc);
        }
    }
    std::cout << "part1 total is: " << uniqueLocations.size() << std::endl;
}
