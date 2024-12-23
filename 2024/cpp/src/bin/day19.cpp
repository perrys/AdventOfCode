#include "lib/file_utils.hpp"
#include "lib/transform.hpp"

#include <assert.h>

#include <algorithm>
#include <functional>
#include <iostream>
#include <ranges>
#include <regex>
#include <unordered_map>
#include <unordered_set>
#include <utility>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 19: Linen Layout
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

using Cache = std::unordered_map<std::string_view, size_t>;

size_t canMatch(std::string_view pattern, Cache& cache,
                const std::unordered_set<std::string_view>& stripes) {
    // std::cout << "\npattern: " << pattern << std::endl;
    if (pattern.empty()) {
        return 1;
    }
    if (cache.contains(pattern)) {
        return cache.at(pattern);
    }
    size_t total = 0;
    for (size_t i = 1; i <= pattern.size(); ++i) {
        std::string_view sv(pattern.begin(), pattern.begin() + i);
        if (stripes.contains(sv)) {
            // std::cout << "matched \"" << sv << "\"" << std::endl;
            total += canMatch({pattern.begin() + i, pattern.end()}, cache, stripes);
        }
    }
    cache[pattern] = total;
    return total;
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
    const std::regex stripeMatcher("(\\w+)");
    std::smatch smatch;
    std::unordered_set<std::string_view> stripes;
    auto start = lines[0].cbegin();
    while (std::regex_search(start, lines[0].cend(), smatch, stripeMatcher)) {
        auto submatch = smatch[1];
        std::string_view sv(submatch.first, submatch.second);
        if (!sv.empty()) {
            stripes.insert(sv);
        }
        start = submatch.second;
    }
    size_t part1Count = 0;
    size_t part2Total = 0;
    Cache cache;
    for (size_t i = 2; i < lines.size(); ++i) {
        size_t total = canMatch(lines[i], cache, stripes);
        if (total > 0) {
            part1Count += 1;
        }
        part2Total += total;
    }
    std::cout << "part 1 answer: " << part1Count << std::endl;
    std::cout << "part 2 answer: " << part2Total << std::endl;
}
