
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

struct BlockGroup {
    size_t start_pos;
    size_t length;
    std::optional<int> id;
};

void parse(const std::string& line, std::vector<BlockGroup>& filelist,
           std::vector<BlockGroup>& freelist) {
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
std::vector<BlockGroup> compressFragment(std::vector<BlockGroup>& filelist,
                                         std::vector<BlockGroup>& freelist) {
    std::vector<BlockGroup> newFiles;
    BlockGroup movedFile;
    std::reverse(freelist.begin(), freelist.end());

    while (true) {
        if (freelist.empty() || filelist.empty()) {
            break;
        }
        BlockGroup& endFile = (*filelist.rbegin());
        BlockGroup& firstFree = (*freelist.rbegin());
        if (endFile.start_pos < firstFree.start_pos) {
            break;
        }
        size_t nblocks = std::min(endFile.length, firstFree.length);
        if (!firstFree.id) {
            firstFree.id = endFile.id.value();
        }
        endFile.length -= nblocks;
        firstFree.length -= nblocks;
        if (0 == endFile.length) {
            filelist.pop_back();
            if (firstFree.length > 0) { // split the free block
                newFiles.emplace_back(firstFree.start_pos, nblocks, firstFree.id);
                firstFree.start_pos += nblocks;
                firstFree.id = {};
            }
        }
        if (0 == firstFree.length) {
            newFiles.emplace_back(firstFree.start_pos, nblocks, firstFree.id);
            freelist.pop_back();
        }
    }
    return newFiles;
}

std::vector<BlockGroup> compressNonFragment(std::vector<BlockGroup>& filelist,
                                            std::vector<BlockGroup>& freelist) {
    std::vector<BlockGroup> newFiles;
    BlockGroup movedFile;

    for (auto& lastFile : std::ranges::reverse_view(filelist)) {
        if (freelist.empty()) {
            break;
        }
        for (auto& freeblock : freelist) {
            if (freeblock.length >= lastFile.length && freeblock.start_pos < lastFile.start_pos) {
                newFiles.emplace_back(freeblock.start_pos, lastFile.length, lastFile.id.value());
                freeblock.length -= lastFile.length;
                freeblock.start_pos += lastFile.length;
                lastFile.length = 0;
                break;
            }
        }
    }
    return newFiles;
}

size_t checksum(const std::vector<BlockGroup>& filelist, const std::vector<BlockGroup>& newFiles) {
    size_t total = 0;
    for (const auto& block : filelist) {
        for (size_t i = 0; i < block.length; ++i) {
            total += (block.start_pos + i) * block.id.value();
        }
    }
    for (const auto& block : newFiles) {
        for (size_t i = 0; i < block.length; ++i) {
            total += (block.start_pos + i) * block.id.value();
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

    std::vector<BlockGroup> filelist;
    std::vector<BlockGroup> freelist;
    parse(contents, filelist, freelist);

    std::vector<BlockGroup> newFiles = compressFragment(filelist, freelist);
    std::cout << "part1 total = " << checksum(filelist, newFiles) << std::endl;

    freelist.clear();
    filelist.clear();
    parse(contents, filelist, freelist);

    newFiles = compressNonFragment(filelist, freelist);
    std::cout << "part2 total = " << checksum(filelist, newFiles) << std::endl;
}
