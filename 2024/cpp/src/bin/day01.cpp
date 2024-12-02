
#include <algorithm>
#include <charconv>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <optional>
#include <ranges>
#include <string_view>
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

namespace scp {

auto getLines(const std::vector<std::string>& args) -> std::optional<std::vector<std::string>> {
    std::filesystem::path datafile(args[1]);
    if (!(std::filesystem::exists(datafile) && std::filesystem::is_regular_file(datafile))) {
        std::cerr << "ERROR: " << datafile << " is not readable." << std::endl;
        return {};
    }
    std::ifstream input(datafile.c_str());
    if (!input.is_open()) {
        std::cerr << "ERROR: " << datafile << " could not be opened." << std::endl;
        return {};
    }
    std::vector<std::string> result;
    std::string line;
    while (std::getline(input, line)) {
        result.push_back(line);
    }
    std::cout << "read " << result.size() << " lines from " << datafile << std::endl;
    return result;
}

template <typename I> auto toInt(const I& subrange) -> std::optional<int> {
    int result{};
    auto [ptr, ec] = std::from_chars(&*subrange.begin(), &*subrange.end(), result);

    if (ec == std::errc()) {
        return result;
    }
    std::cerr << "ERROR: invalid integer \"" << ptr << "\", "
              << std::make_error_condition(ec).message() << std::endl;
    return {};
}

} // namespace scp

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }

    const auto optlines = scp::getLines(arguments);
    if (!optlines) {
        return -1;
    }

    std::vector<int> first;
    std::vector<int> second;
    size_t count = 0;
    for (auto line : optlines.value() //
                         | std::ranges::views::filter([](auto s) { return !s.empty(); })) {
        auto numbers =                                                      //
            line                                                            //
            | std::ranges::views::split(std::string(" "))                   //
            | std::ranges::views::filter([](auto s) { return !s.empty(); }) //
            | std::ranges::views::transform([](auto s) { return scp::toInt(s).value(); });

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
