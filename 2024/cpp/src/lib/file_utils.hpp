#include <filesystem>
#include <string>
#include <vector>

namespace scp {
auto getLines(const std::filesystem::path datafile) -> std::vector<std::string>;
}
