
#include "lib/file_utils.hpp"
#include "lib/grid.hpp"
#include "lib/hash_utils.hpp"
#include "lib/transform.hpp"

#include <assert.h>

#include <algorithm>
#include <iostream>
#include <numeric>
#include <optional>
#include <ranges>
#include <unordered_map>
#include <unordered_set>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 12: Garden Groups
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

struct Region {
    char kind;
    std::vector<scp::Coordinate> tiles;
    void consume(Region& other) {
        assert(this->kind == other.kind);
        std::copy(other.tiles.begin(), other.tiles.end(), std::back_inserter(this->tiles));
        other.tiles.clear();
    }
    size_t area() const {
        return this->tiles.size();
    }
    size_t perimeter(const scp::Grid& grid) const {
        const std::array<scp::Direction, 4> directions{scp::NORTH, scp::SOUTH, scp::EAST,
                                                       scp::WEST};
        size_t perimeter = 0;
        for (auto coord : tiles) {
            for (auto dir : directions) {
                auto otherKind = grid.getWithOffsets(coord, dir);
                if (!otherKind.has_value() || otherKind.value() != this->kind) {
                    perimeter += 1;
                }
            }
        }
        return perimeter;
    }
};

using Regions = std::vector<Region>;

} // namespace

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }

    auto lines = scp::getLines(arguments[1]);
    scp::Grid grid(std::move(lines));

    Regions regions;
    std::vector<std::vector<size_t>> regionMap(grid.height());

    for (size_t iy = 0; iy < grid.height(); ++iy) {
        for (size_t ix = 0; ix < grid.width(); ++ix) {
            scp::Coordinate coord{ix, iy};
            auto myKind = grid.get(coord).value();
            std::optional<size_t> myRegionId;

            auto dir = scp::NORTH;
            auto northKind = grid.getWithOffsets(coord, dir);
            if (northKind.has_value() && northKind.value() == myKind) {
                myRegionId = regionMap.at(iy + dir.dy).at(ix + dir.dx);
            }

            dir = scp::WEST;
            auto westKind = grid.getWithOffsets(coord, dir);
            if (westKind.has_value() && westKind.value() == myKind) {
                auto westRegionId = regionMap.at(iy + dir.dy).at(ix + dir.dx);
                if (myRegionId.has_value()) {
                    if (myRegionId.value() != westRegionId) {
                        // north and west regions need to be joined
                        for (auto loc : regions.at(westRegionId).tiles) {
                            regionMap.at(loc.iy).at(loc.ix) = myRegionId.value();
                        }
                        regions.at(myRegionId.value()).consume(regions.at(westRegionId));
                    }
                } else {
                    myRegionId = westRegionId;
                }
            }

            if (!myRegionId.has_value()) {
                myRegionId = regions.size();
                regions.push_back({myKind, {}});
            }
            regionMap.at(iy).push_back(myRegionId.value());
            regions.at(myRegionId.value()).tiles.push_back(coord);
        }
    }

    // for (auto line : gridMap) {
    //     for (auto region : line) {
    //         std::cout << region << ", ";
    //     }
    //     std::cout << "\n";
    // }

    size_t total = 0;
    for (auto region : regions) {
        // std::cout << idx << "-----" << region.kind << std::endl;
        // ++idx;
        // for (auto tile : region.tiles) {
        //     std::cout << tile << std::endl;
        // }
        // std::cout << "area: " << region.area() << ", perim: " << region.perimeter(grid)
        //           << std::endl;
        total += region.area() * region.perimeter(grid);
    }
    std::cout << "part1 total: " << total << std::endl;
}
