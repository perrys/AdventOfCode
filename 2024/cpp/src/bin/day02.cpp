
#include "lib/file_utils.hpp"
#include "lib/transform.hpp"

#include <algorithm>
#include <iostream>
#include <optional>
#include <ranges>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 2: Red-Nosed Reports
 *
 * See <https://adventofcode.com/2024/day/1>
 */

namespace {

template <typename R> auto part1Test(const R& numbers) -> bool {
    std::optional<bool> increasing{};
    for (auto i = numbers.begin() + 1; i != numbers.end(); ++i) {
        int diff = *i - *(i - 1);
        if (diff == 0 || std::abs(diff) > 3) {
            return false;
        }
        bool inc = diff > 0;
        if (increasing.has_value()) {
            if (inc != increasing.value()) {
                return false;
            }
        } else {
            increasing = inc;
        }
    }
    return true;
}

auto part2Test(const std::vector<int>& numbers) -> bool {
    if (part1Test(numbers)) {
        return true;
    }
    for (size_t i = 0; i < numbers.size(); ++i) {
        // implementing a skip iterator is too much work and would make the following *less*
        // understandable:
        std::vector<int> copy(numbers);
        copy.erase(copy.begin() + i);
        if (part1Test(copy)) {
            return true;
        }
    }
    return false;
}

} // namespace

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }

    const auto lines = scp::getLines(arguments[1]);

    size_t p1count = 0, p2count = 0;
    for (auto line : lines | std::ranges::views::filter([](auto s) { return !s.empty(); })) {
        auto tokens = line                                                            //
                      | std::ranges::views::split(std::string(" "))                   //
                      | std::ranges::views::filter([](auto s) { return !s.empty(); }) //
                      | std::ranges::views::transform(scp::parseInt());

        auto vec = std::vector(tokens.begin(), tokens.end());
        if (part1Test(vec)) {
            ++p1count;
        }
        if (part2Test(vec)) {
            ++p2count;
        }
    }

    std::cout << "part1 result: " << p1count << std::endl;
    std::cout << "part2 result: " << p2count << std::endl;
}
