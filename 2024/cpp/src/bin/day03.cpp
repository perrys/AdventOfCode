
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
 * Day 3: Mull It Over
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

const auto npos = std::string::npos;

auto parseMuls(const std::string_view line) {
    const std::string mul("mul(");
    auto parseInt = [&line](size_t start, size_t end) {
        return scp::parseInt::parse(line.data() + start, line.data() + end);
    };
    int total = 0;
    size_t pos = 0, start, end;
    while ((pos = line.find(mul, pos)) != npos) {
        pos += mul.length();
        start = pos;
        // hardcoded recursive descent :)
        if ((end = line.find(',', start)) != npos) {
            auto number1 = parseInt(start, end);
            if (number1) {
                start = end + 1;
                if ((end = line.find(')', start)) != npos) {
                    auto number2 = parseInt(start, end);
                    if (number2) {
                        total += number1.value() * number2.value();
                        pos = end + 1;
                    }
                }
            }
        }
    }
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

    const auto contents = scp::getContents(arguments[1]);
    const int part1Total = parseMuls(contents);
    std::cout << "part1 total: " << part1Total << std::endl;

    int part2Total = 0;
    size_t start = 0, end;
    const std::string doCmd("do()");
    const std::string dontCmd("don't()");

    while ((end = contents.find(dontCmd, start)) != npos) {
        part2Total += parseMuls({contents.data() + start, contents.data() + end});
        end += dontCmd.length();
        start = contents.find("do()", end);
        if (npos == start) {
            break;
        }
    }
    if (start != npos) {
        part2Total += parseMuls(
            std::string_view(contents.data() + start, contents.data() + contents.length()));
    }

    std::cout << "part2 total: " << part2Total << std::endl;
}
