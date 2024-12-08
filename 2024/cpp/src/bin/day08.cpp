
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

using Int = int64_t;
using Coord = scp::GenCoordinate<Int>;
using CoordList = std::vector<Coord>;

struct BoundsCheck {
    Int width;
    Int height;
    bool operator()(const Coord& c) const {
        return c.ix >= 0 && c.ix < width && c.iy >= 0 && c.iy < height;
    }
};

void addAntinodes(Coord a, Coord b, CoordList& result) {
    auto [dx, dy] = a.displacement(b);
    result.emplace_back(a.ix + 2 * dx, a.iy + 2 * dy);
    result.emplace_back(b.ix - 2 * dx, b.iy - 2 * dy);
}

void addAllAntinodes(Coord a, Coord b, const BoundsCheck& filt, CoordList& result) {
    auto [dx, dy] = a.displacement(b);
    Coord copy = a;
    while (filt(copy)) {
        result.push_back(copy);
        copy = {copy.ix + dx, copy.iy + dy};
    }
    copy = b;
    while (filt(copy)) {
        result.push_back(copy);
        copy = {copy.ix - dx, copy.iy - dy};
    }
}

void calculateNodeLocations(const CoordList::const_iterator begin,
                            const CoordList::const_iterator end, const BoundsCheck& filt,
                            bool isPart2, CoordList& result) {
    if (begin == end) {
        return;
    }
    const Coord myloc = *begin;
    for (auto iter = begin + 1; iter != end; ++iter) {
        if (isPart2) {
            addAllAntinodes(myloc, (*iter), filt, result);
        } else {
            addAntinodes(myloc, (*iter), result);
        }
    }
    calculateNodeLocations(begin + 1, end, filt, isPart2, result);
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
    const Int height = lines.size();
    Int width = 0;
    std::unordered_map<char, CoordList> map;

    for (Int iy = 0; iy < height; ++iy) {
        const auto line = lines[iy];
        if (0 == iy) {
            width = line.size();
        } else {
            assert(static_cast<Int>(line.size()) == width); // irregular grid
        }
        for (Int ix = 0; ix < static_cast<Int>(line.size()); ++ix) {
            const char c = line[ix];
            switch (c) {
            case '.':
                break;
            default:
                auto [iter, _flag] = map.try_emplace(c);
                (*iter).second.push_back({ix, iy});
            }
        }
    }

    const BoundsCheck filt{width, height};
    std::unordered_set<Coord> part1Locations;
    std::unordered_set<Coord> part2Locations;

    for (auto frequency : map) {
        const auto& [_freq, locs] = frequency;
        {
            CoordList antinodeLocations;
            calculateNodeLocations(locs.begin(), locs.end(), filt, false, antinodeLocations);
            for (auto loc : antinodeLocations | std::ranges::views::filter(filt)) {
                part1Locations.insert(loc);
            }
        }
        {
            CoordList antinodeLocations;
            calculateNodeLocations(locs.begin(), locs.end(), filt, true, antinodeLocations);
            for (auto loc : antinodeLocations) {
                part2Locations.insert(loc);
            }
        }
    }
    std::cout << "part1 total is: " << part1Locations.size() << std::endl;
    std::cout << "part2 total is: " << part2Locations.size() << std::endl;
}
