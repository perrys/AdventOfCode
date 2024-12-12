
#include "lib/file_utils.hpp"
#include "lib/hash_utils.hpp"
#include "lib/transform.hpp"

#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <iostream>
#include <optional>
#include <ranges>
#include <unordered_map>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 11: Plutonian Pebbles
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

size_t countDigits(size_t n) {
    size_t count = 0;
    while (n > 0) {
        n /= 10;
        ++count;
    }
    return count;
}

bool evenDigits(size_t n) {
    return !(countDigits(n) & 0x1);
}

std::pair<size_t, size_t> splitDigits(size_t n) {
    size_t ndigits = countDigits(n);
    size_t factor = std::pow(10, (ndigits / 2));
    size_t top = n / factor;
    size_t bottom = n - (top * factor);
    return {top, bottom};
}

std::vector<size_t> iterate(const std::vector<size_t>& tokens) {
    std::vector<size_t> result;
    result.reserve(tokens.size() * 2);
    for (auto token : tokens) {
        if (0 == token) {
            result.push_back(1);
        } else if (evenDigits(token)) {
            auto [t1, t2] = splitDigits(token);
            result.push_back(t1);
            result.push_back(t2);
        } else {
            result.push_back(token * 2024);
        }
    }
    return result;
}

size_t recurse(const size_t token, const size_t remainingBlinks,
               std::unordered_map<std::pair<size_t, size_t>, size_t>& memos) {
    if (0 == remainingBlinks) {
        return 1;
    }
    auto iter = memos.find({token, remainingBlinks});
    if (iter != memos.end()) {
        return (*iter).second;
    }
    size_t decBlinks = remainingBlinks - 1;
    size_t result;
    if (0 == token) {
        result = recurse(1, decBlinks, memos);
    } else if (evenDigits(token)) {
        auto [t1, t2] = splitDigits(token);
        result = recurse(t1, decBlinks, memos) + recurse(t2, decBlinks, memos);
    } else {
        result = recurse(token * 2024, decBlinks, memos);
    }
    memos.emplace(std::make_pair(token, remainingBlinks), result);
    return result;
}

} // namespace

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }

    const auto contents = scp::getContents(arguments[1]);
    auto range = contents                                                        //
                 | std::ranges::views::split(std::string_view(" "))              //
                 | std::ranges::views::filter([](auto s) { return !s.empty(); }) //
                 | std::ranges::views::transform(scp::Parser<size_t>());

    auto tokens = std::vector(range.begin(), range.end());
    const size_t nIters = 25;
    const bool debug = false;
    for (size_t i = 0; i < nIters; ++i) {
        tokens = iterate(tokens);
        if (debug) {
            for (auto tok : tokens) {
                std::cout << tok << " ";
            }
            std::cout << std::endl;
        }
    }
    std::cout << "part1 answer: " << tokens.size() << std::endl;

    std::unordered_map<std::pair<size_t, size_t>, size_t> memos;
    size_t part2Result = 0;
    tokens = std::vector(range.begin(), range.end());
    for (auto token : tokens) {
        part2Result += recurse(token, 75, memos);
    }
    // for (auto kv : memos) {
    //     std::cout << "n: " << kv.first.first << ", remain:" << kv.first.second
    //               << ", result: " << kv.second << std::endl;
    // }
    std::cout << "part2 answer: " << part2Result << std::endl;
}
