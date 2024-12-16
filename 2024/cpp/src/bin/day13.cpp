
#include "lib/file_utils.hpp"
#include "lib/numerical_recipes/nr.hpp"
#include "lib/transform.hpp"

#include <assert.h>

#include <algorithm>
#include <cmath>
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
 * Day 13: Claw Contraption
 *
 * See <https://adventofcode.com/2024>
 */

namespace {
const std::regex buttonMatch("Button [AB]: X\\+(\\d+), Y\\+(\\d+)");
const std::regex targetMatch("Prize: X=(\\d+), Y=(\\d+)");

struct Block {
    std::vector<int> xmoves;
    std::vector<int> ymoves;
    std::vector<int> xytarget;

    static Block parse(const std::vector<std::string>& lines, size_t idx) {
        Block result;
        assert(lines[idx].starts_with("Button A"));
        std::smatch smatch;

        assert(std::regex_match(lines[idx++], smatch, buttonMatch));
        result.xmoves.push_back(
            scp::parseInt::parse(&*smatch[1].first, &*smatch[1].second).value());
        result.xmoves.push_back(
            scp::parseInt::parse(&*smatch[2].first, &*smatch[2].second).value());

        assert(std::regex_match(lines[idx++], smatch, buttonMatch));
        result.ymoves.push_back(
            scp::parseInt::parse(&*smatch[1].first, &*smatch[1].second).value());
        result.ymoves.push_back(
            scp::parseInt::parse(&*smatch[2].first, &*smatch[2].second).value());

        assert(std::regex_match(lines[idx++], smatch, targetMatch));
        result.xytarget.push_back(
            scp::parseInt::parse(&*smatch[1].first, &*smatch[1].second).value());
        result.xytarget.push_back(
            scp::parseInt::parse(&*smatch[2].first, &*smatch[2].second).value());

        return result;
    }

    size_t solve() {
        std::vector<double> coefficients{
            static_cast<double>(this->xmoves[0]), static_cast<double>(this->ymoves[0]),
            static_cast<double>(this->xmoves[1]), static_cast<double>(this->ymoves[1])};
        std::vector<double> targets{static_cast<double>(this->xytarget[0]),
                                    static_cast<double>(this->xytarget[1])};
        nr::guassJordanElimination(coefficients.data(), targets.data(), 2);
        // targets now holds the result (double-precision)
        for (int i = 0; i < 2; ++i) {
            const auto rounded = std::round(targets[i]);

            if (std::abs(rounded - targets[i]) > 0.0000001 || targets[i] < 0 || targets[i] > 100) {
                // no exact solution
                return 0;
            }
        }
        const size_t result = static_cast<size_t>(std::llround(targets[0])) * 3 +
                              static_cast<size_t>(std::llround(targets[1]));
        return result;
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

    size_t part1Total = 0;
    for (size_t i = 0; i < lines.size(); i += 4) {
        auto block = Block::parse(lines, i);
        part1Total += block.solve();
    }
    std::cout << "part1 answer: " << part1Total << std::endl;
}
