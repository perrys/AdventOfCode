
#include "lib/file_utils.hpp"
#include "lib/transform.hpp"

#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <iostream>
#include <optional>
#include <ranges>
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

} // namespace

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }

    const auto contents = scp::getContents(arguments[1]);
    const size_t nIters = 25;
    const bool debug = false;

    auto range = contents                                                        //
                 | std::ranges::views::split(std::string_view(" "))              //
                 | std::ranges::views::filter([](auto s) { return !s.empty(); }) //
                 | std::ranges::views::transform(scp::Parser<size_t>());

    for (size_t j = 1; j <= nIters; ++j) {
        auto tokens = std::vector(range.begin(), range.end());
        for (size_t i = 0; i < j; ++i) {
            tokens = iterate(tokens);
            if (debug) {
                for (auto tok : tokens) {
                    std::cout << tok << " ";
                }
                std::cout << std::endl;
            }
        }
        std::cout << j << " answer: " << tokens.size() << std::endl;
    }
}
