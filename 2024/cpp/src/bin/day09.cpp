
#include "lib/file_utils.hpp"
#include "lib/transform.hpp"

#include <algorithm>
#include <cassert>
#include <cstdint>
#include <iostream>
#include <optional>
#include <ranges>
#include <vector>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 9: Disk Fragmenter
 *
 * See <https://adventofcode.com/2024>
 */

namespace {
struct FileBlock {
    size_t start_pos;
    size_t length;
    std::optional<int> id;
};

void parse(const std::string& line, std::vector<FileBlock>& filelist,
           std::vector<FileBlock>& freelist) {
    size_t startPos = 0;
    size_t fileId = 0;
    for (size_t i = 0; i < line.length(); ++i) {
        const std::int8_t fileSize = line[i] - '0';
        assert(fileSize >= 0 && fileSize < 10);
        const bool isfree = i & 0x01;
        if (isfree) {
            freelist.emplace_back(startPos, fileSize);
        } else {
            filelist.emplace_back(startPos, fileSize, fileId);
            ++fileId;
        }
        startPos += fileSize;
    }
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
    std::vector<FileBlock> filelist;
    std::vector<FileBlock> freelist;
    parse(contents, filelist, freelist);
    std::reverse(freelist.begin(), freelist.end());
    std::vector<FileBlock> newFiles;
    FileBlock movedFile;

    while (true) {
        if (freelist.empty() || filelist.empty()) {
            break;
        }
        FileBlock& lastFile = (*filelist.rbegin());
        FileBlock& nextFree = (*freelist.rbegin());
        if (lastFile.start_pos < nextFree.start_pos) {
            break;
        }
        size_t nblocks = std::min(lastFile.length, nextFree.length);
        if (!nextFree.id) {
            nextFree.id = lastFile.id.value();
        }
        lastFile.length -= nblocks;
        nextFree.length -= nblocks;
        if (0 == lastFile.length) {
            filelist.pop_back();
            if (nextFree.length > 0) { // split the free block
                newFiles.emplace_back(nextFree.start_pos, nextFree.length, nextFree.id);
                nextFree.start_pos += nextFree.length;
                nextFree.id = {};
            }
        }
        if (0 == nextFree.length) {
            newFiles.emplace_back(nextFree.start_pos, nextFree.length, nextFree.id);
            freelist.pop_back();
        }
    }
    size_t part1Total = 0;
    for (const auto& block : filelist) {
        for (size_t i = 0; i < block.length; ++i) {
            part1Total += block.start_pos + i * block.id.value();
            std::cout << "(" << block.id.value() << ", " << block.start_pos + i << ")" << std::endl;
        }
    }
    for (const auto& block : newFiles) {
        for (size_t i = 0; i < block.length; ++i) {
            part1Total += block.start_pos + i * block.id.value();
            std::cout << "(" << block.id.value() << ", " << block.start_pos + i << ")" << std::endl;
        }
    }
    std::cout << "part1 total = " << part1Total << std::endl;
}
