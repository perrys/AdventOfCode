#include <stdlib.h>
#include <string.h>

#include <algorithm>
#include <charconv>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <optional>
#include <ranges>
#include <string>
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
    if (args.size() != 2) {
        std::filesystem::path progname(args[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }
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

template <typename T> auto toInt(const T& str) -> std::optional<int> {
    int result{};
    auto [ptr, ec] = std::from_chars(str.data(), str.data() + str.size(), result);

    if (ec == std::errc()) {
        return result;
    }
    std::cerr << "ERROR: invalid integer \"" << ptr << "\", "
              << std::make_error_condition(ec).message() << std::endl;
    return {};
}

template <typename S> auto tokenize(const S& str) -> std::vector<std::string> {
    std::string copy(str.begin(), str.end());
    std::vector<std::string> result;
    char* ptr = copy.data();
    char* saveptr = nullptr;
    while (ptr <= copy.data() + copy.size()) {
        char* token = ::strtok_r(ptr, " \t", &saveptr);
        if (nullptr == token) {
            break;
        }
        result.push_back(token);
        ptr = nullptr;
    }
    return result;
}

} // namespace scp

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    const auto optlines = scp::getLines(arguments);
    if (!optlines) {
        return -1;
    }

    std::vector<int> first;
    std::vector<int> second;
    size_t count = 0;
    for (auto line :
         optlines.value() | std::ranges::views::filter([](auto s) { return !s.empty(); })) {
        auto tokens = scp::tokenize(line);
        if (tokens.size() != 2) {
            std::cerr << "ERROR: invalid input at line " << count << std::endl;
            ::exit(1);
        }
        first.push_back(scp::toInt(tokens[0]).value());
        second.push_back(scp::toInt(tokens[1]).value());
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
    std::ranges::for_each(first, [&occurences](auto val) {
        auto n = occurences[val];
        occurences[val] = n + 1;
    });
    total = 0;
    for (size_t i = 0; i < first.size(); ++i) {
        total += first[i] * occurences[first[i]];
    }
    std::cout << "part1 result: " << total << std::endl;
}
