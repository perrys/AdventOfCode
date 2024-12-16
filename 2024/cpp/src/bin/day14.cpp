
#include "lib/file_utils.hpp"
#include "lib/grid.hpp"
#include "lib/numerical_recipes/nr.hpp"
#include "lib/transform.hpp"

#include <assert.h>

#include <algorithm>
#include <cmath>
#include <functional>
#include <iostream>
#include <numeric>
#include <optional>
#include <ranges>
#include <regex>
#include <unordered_map>
#include <unordered_set>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 14: Restroom Redoubt
 *
 * See <https://adventofcode.com/2024>
 */

namespace {
const std::regex matcher("p=(-?\\d+),(-?\\d+) v=(-?\\d+),(-?\\d+)");

using Coord = scp::Coordinate;

struct Robot {
    int px, py, dx, dy;

    static Robot parse(const std::string& line) {
        Robot result;
        assert(line.starts_with("p="));
        std::smatch smatch;

        assert(std::regex_match(line, smatch, matcher));
        result.px = scp::parseInt::parse(&*smatch[1].first, &*smatch[1].second).value();
        result.py = scp::parseInt::parse(&*smatch[2].first, &*smatch[2].second).value();
        result.dx = scp::parseInt::parse(&*smatch[3].first, &*smatch[3].second).value();
        result.dy = scp::parseInt::parse(&*smatch[4].first, &*smatch[4].second).value();
        return result;
    }

    Coord move(int width, int height, int turns) const {
        int fx = (this->px + this->dx * turns) % width;
        int fy = (this->py + this->dy * turns) % height;
        return {static_cast<size_t>(fx < 0 ? width + fx : fx),
                static_cast<size_t>(fy < 0 ? height + fy : fy)};
    }
};

} // namespace

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }
    auto lines = scp::getLines(arguments[1]);

    const size_t width = 101;
    const size_t height = 103;
    const size_t turns = 100;

    const size_t left = width / 2;
    const size_t right = left + 1;
    const size_t top = height / 2;
    const size_t bottom = top + 1;

    std::array<size_t, 4> quadrants{0, 0, 0, 0};
    for (auto line : lines) {
        auto robot = Robot::parse(line);
        Coord pos = robot.move(width, height, turns);
        if (pos.ix < left && pos.iy < top) {
            quadrants[0] += 1;
        } else if (pos.ix >= right && pos.iy < top) {
            quadrants[1] += 1;
        } else if (pos.ix < left && pos.iy >= bottom) {
            quadrants[2] += 1;
        } else if (pos.ix >= right && pos.iy >= bottom) {
            quadrants[3] += 1;
        }
    }
    std::cout << "part1 total: "
              << std::accumulate(quadrants.begin(), quadrants.end(), 1, std::multiplies<>())
              << std::endl;
}
