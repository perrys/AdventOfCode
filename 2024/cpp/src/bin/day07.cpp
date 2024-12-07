
#include "lib/file_utils.hpp"
#include "lib/transform.hpp"

#include <assert.h>
#include <stdio.h>
#include <string.h>

#include <algorithm>
#include <iostream>
#include <optional>
#include <ranges>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 7: Bridge Repair
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

const auto npos = std::string::npos;

using Int = int64_t;
using IntVec = std::vector<Int>;

Int concatenate(Int lhs, Int rhs) {
    // considered doing this numerically with log10 but snprintf is fast enough
    // NB std::stringstream is really slooooow
    char buf[256];
    snprintf(buf, 256, "%ld%ld", lhs, rhs);
    return scp::Parser<Int>::parse(buf, buf + strlen(buf)).value();
}

IntVec calculateTree(IntVec::const_reverse_iterator start, IntVec::const_reverse_iterator end,
                     bool isPart2) {
    Int mynum = *(start);
    start += 1;
    if (start == end) {
        return {mynum};
    }

    const auto result = calculateTree(start, end, isPart2);
    IntVec retval;
    retval.reserve(result.size() * (isPart2 ? 3 : 2));
    for (auto n : result) {
        retval.push_back(n + mynum);
        retval.push_back(n * mynum);
        if (isPart2) {
            retval.push_back(concatenate(n, mynum)); // reversed!
        }
    }
    return retval;
}

struct Eqn {
    Int answer;
    IntVec numbers;

    static Eqn parse(const std::string& line) {
        auto idx = line.find(':');
        assert(idx != npos);
        auto answer = scp::Parser<Int>::parse(line.data(), line.data() + idx);
        assert(answer.has_value());
        auto iter = std::string_view(line.data() + idx + 1, line.data() + line.size()) //
                    | std::ranges::views::split(std::string(" "))                      //
                    | std::ranges::views::filter([](auto s) { return !s.empty(); })    //
                    | std::ranges::views::transform(scp::parseInt());
        IntVec numbers(iter.begin(), iter.end());
        return {answer.value(), std::move(numbers)};
    }

    bool isValid(bool isPart2 = false) const {
        size_t count = 0;
        // numbers are evaluated left-to-right, so we have to recurse right-to-left
        for (auto n : calculateTree(this->numbers.rbegin(), this->numbers.rend(), isPart2)) {
            if (n == this->answer) {
                return true;
            }
            ++count;
        }
        return false;
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
    Int part1Total = 0;
    Int part2Total = 0;
    for (const auto& line : lines) {
        const auto eqn = Eqn::parse(line);
        if (eqn.isValid()) {
            //std::cout << "valid: " << line << std::endl;
            part1Total += eqn.answer;
        }
        if (eqn.isValid(true)) {
            part2Total += eqn.answer;
        }
    }
    std::cout << "part1 answer: " << part1Total << std::endl;
    std::cout << "part2 answer: " << part2Total << std::endl;
}
