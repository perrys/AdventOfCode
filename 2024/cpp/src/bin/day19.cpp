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

std::optional<std::string> canMatch(std::string_view pattern,
                                    const std::unordered_set<std::string_view>& stripes) {
    // std::cout << "\npattern: " << pattern << std::endl;
    if (pattern.empty()) {
        return "";
    }
    for (size_t i = 1; i <= pattern.size(); ++i) {
        std::string_view sv(pattern.begin(), pattern.begin() + i);
        if (stripes.contains(sv)) {
            // std::cout << "matched \"" << sv << "\"" << std::endl;
            auto lower = canMatch({pattern.begin() + i, pattern.end()}, stripes);
            if (lower.has_value()) {
                return std::string(sv) + lower.value();
            }
        }
    }
    return {};
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
    size_t count = 0;
    for (size_t i = 2; i < lines.size(); ++i) {
        auto result = canMatch(lines[i], stripes);
        if (result) {
            assert(result == lines[i]);
            count += 1;
        }
    }
    std::cout << "part 1 answer: " << count << std::endl;
}
