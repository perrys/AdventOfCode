#include "file_utils.hpp"

#include <charconv>
#include <fstream>
#include <iostream>

namespace scp {

auto getContents(std::filesystem::path datafile) -> std::string {
    if (!(std::filesystem::exists(datafile) && std::filesystem::is_regular_file(datafile))) {
        std::cerr << "ERROR: " << datafile << " is not readable." << std::endl;
        return {};
    }
    std::ifstream input(datafile.c_str());
    if (!input.is_open()) {
        std::cerr << "ERROR: " << datafile << " could not be opened." << std::endl;
        return {};
    }
    std::ostringstream sstr;
    sstr << input.rdbuf();
    return sstr.str();
}

auto getLines(std::filesystem::path datafile) -> std::vector<std::string> {
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
} // namespace scp
