
#include "lib/file_utils.hpp"
#include "lib/transform.hpp"

#include <assert.h>

#include <algorithm>
#include <filesystem>
#include <iostream>
#include <optional>
#include <ranges>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 5: Print Queue
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

const auto npos = std::string::npos;
const scp::parseInt parser;

using Update = std::vector<int>;
struct Rule {
    int first;
    int second;

    Rule(std::string_view s1, std::string_view s2) : first(parser(s1)), second(parser(s2)) {
    }

    template <typename I> static Rule create(I& subrange) {
        auto iter = subrange.begin();

        std::string_view first((*iter).begin(), (*iter).end());
        ++iter;
        std::string_view second((*iter).begin(), (*iter).end());
        return Rule(first, second);
    };

    bool operator()(const Update& line) const {
        std::optional<int> n1{}, n2{};
        for (auto num : line) {
            if (num == this->first) {
                if (n2) {
                    return false;
                }
                n1 = num;
            }
            if (num == this->second) {
                if (n1) {
                    return true;
                }
                n2 = num;
            }
        }
        assert(!(n1 && n2));
        return true;
    }
};

struct Sorter {
    std::vector<Rule>& rules;
    bool operator()(const int lhs, const int rhs) {
        for (auto rule : rules) {
            if (rule.first == lhs && rule.second == rhs) {
                return true;
            }
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

    const auto lines = scp::getLines(arguments[1]);
    std::vector<Rule> rules;
    for (auto line :
         lines | std::ranges::views::filter([](auto s) { return s.find('|') != npos; })) {
        auto pts = line | std::ranges::views::split(std::string("|"));
        rules.push_back(Rule::create(pts));
    }

    std::vector<Update> updates;
    for (auto line :
         lines | std::ranges::views::filter([](auto s) { return s.find(',') != npos; })) {

        auto pts = line | std::ranges::views::split(std::string(","));
        Update updateLine;
        for (auto numStr : pts) {
            updateLine.push_back(parser(numStr));
        }
        updates.push_back(std::move(updateLine));
    }

    std::cout << "parsed " << rules.size() << " rules and " << updates.size() << " update lines"
              << std::endl;

    int part1Total = 0;
    int part2Total = 0;
    for (auto line : updates) {
        bool okay = true;
        for (auto rule : rules) {
            if (!rule(line)) {
                okay = false;
                break;
            }
        }
        assert((line.size() & 1));
        if (okay) {
            part1Total += line.at(line.size() / 2);
        } else {
            Sorter sorter{rules};
            std::ranges::sort(line, sorter);
            part2Total += line.at(line.size() / 2);
        }
    }
    std::cout << "part1 answer: " << part1Total << std::endl;
    std::cout << "part2 answer: " << part2Total << std::endl;
}
