
#include "lib/file_utils.hpp"
#include "lib/transform.hpp"

#include <algorithm>
#include <iostream>
#include <ranges>
#include <unordered_map>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 1: Historian Hysteria
 *
 * See <https://adventofcode.com/2024/day/1>
 */

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }

    const auto lines = scp::getLines(arguments[1]);
    std::vector<int> first;
    std::vector<int> second;
    size_t count = 0;
    for (auto line : lines | std::ranges::views::filter([](auto s) { return !s.empty(); })) {
        auto numbers =                                                      //
            line                                                            //
            | std::ranges::views::split(std::string(" "))                   //
            | std::ranges::views::filter([](auto s) { return !s.empty(); }) //
            | std::ranges::views::transform(scp::parseInt());

        auto numvec = std::vector(numbers.begin(), numbers.end());
        if (numvec.size() != 2) {
            std::cerr << "ERROR: invalid input at line " << count << ", read " << numvec.size()
                      << " numbers" << std::endl;
            return -1;
        }
        first.push_back(numvec[0]);
        second.push_back(numvec[1]);
        ++count;
    }

    std::ranges::sort(first);
    std::ranges::sort(second);

    int total = 0;
    for (size_t i = 0; i < first.size(); ++i) {
        total += std::abs(second[i] - first[i]);
    }
    std::cout << "part1 result: " << total << std::endl;

    std::unordered_map<int, size_t> occurences;
    std::ranges::for_each(second, [&occurences](auto val) {
        auto n = occurences[val];
        occurences[val] = n + 1;
    });
    total = 0;
    for (size_t i = 0; i < first.size(); ++i) {
        total += first[i] * occurences[first[i]];
    }
    std::cout << "part2 result: " << total << std::endl;
}
