
#include "lib/file_utils.hpp"
#include "lib/transform.hpp"

#include <assert.h>

#include <algorithm>
#include <cmath>
#include <functional>
#include <iostream>
#include <limits>
#include <numeric>
#include <ranges>
#include <unordered_map>
#include <unordered_set>
#include <utility>

/**
 * /file
 *
 * Advent of code challenge 2024.
 * Day 17: Chronospatial Computer
 *
 * See <https://adventofcode.com/2024>
 */

namespace {

struct CPU {
    int regA;
    int regB;
    int regC;
    size_t pc;

    static auto parse(std::vector<std::string>& lines) -> CPU {
        // auto parser = [](const std::string& line) -> int {
        //     auto range = line                                                                 //
        //                  | std::ranges::views::filter([](auto c) { return std::isdigit(c); }) //
        //                  | std::ranges::views::transform(scp::parseInt());
        //     return *(range.begin());
        // };
        auto parser = [](const std::string& line) -> int {
            std::vector<char> digits;
            for (auto c : line) {
                if (std::isdigit(c)) {
                    digits.push_back(c);
                }
            }
            const char* start = &digits[0];
            return scp::parseInt::parse(start, start + digits.size()).value();
        };
        return {parser(lines[0]), parser(lines[1]), parser(lines[2]), 0};
    }

    int comboOperand(int val) {
        switch (val) {
        case 0:
        case 1:
        case 2:
        case 3:
            return val;
        case 4:
            return this->regA;
        case 5:
            return this->regB;
        case 6:
            return this->regC;
        default:
            throw std::logic_error("invalid operand: " + std::to_string(val));
        }
    }

    std::ostream& output(std::ostream& ostream) {
        ostream << "[" << this->pc << "] A: " << this->regA << ", B: " << this->regB
                << ", C: " << this->regC << std::endl;
        return ostream;
    }

    auto execute(const std::vector<int>& program) -> std::vector<int> {
        const bool debug = false;
        if (debug) {
            this->output(std::cout);
        }
        std::vector<int> output;

        while (this->pc < program.size()) {
            auto opcode = program.at(this->pc);
            auto operand = program.at(this->pc + 1);
            assert(opcode >= 0 && opcode <= 7);
            assert(operand >= 0 && operand <= 7);
            switch (opcode) {
            case 0: // adv

                this->regA = this->regA / (1 << comboOperand(operand));
                this->pc += 2;
                break;
            case 1: // bxl
                this->regB = this->regB ^ operand;
                this->pc += 2;
                break;
            case 2: // bst
                this->regB = comboOperand(operand) % 8;
                this->pc += 2;
                break;
            case 3: // jnz
                if (this->regA != 0) {
                    this->pc = operand;
                } else {
                    this->pc += 2;
                }
                break;
            case 4: // bxc
                this->regB = this->regB ^ this->regC;
                this->pc += 2;
                break;
            case 5: // out
                output.push_back(comboOperand(operand) % 8);
                this->pc += 2;
                break;
            case 6: // bdv
                this->regB = this->regA / (1 << comboOperand(operand));
                this->pc += 2;
                break;
            case 7: // cdv
                this->regC = this->regA / (1 << comboOperand(operand));
                this->pc += 2;
                break;
            }
            if (debug) {
                this->output(std::cout);
            }
        }
        return output;
    }
};
} // namespace

std::ostream& operator<<(std::ostream& ostream, const std::vector<int>& vals) {
    bool first = true;
    for (auto i : vals) {
        if (first) {
            first = false;
        } else {
            std::cout << ",";
        }
        std::cout << i;
    }
    return ostream;
}

int main(int argc, char* argv[]) {
    std::vector<std::string> arguments(argv, argv + argc);
    if (arguments.size() != 2) {
        std::filesystem::path progname(arguments[0]);
        std::cerr << "USAGE: " << progname.filename() << " <filename.dat>" << std::endl;
        return {};
    }

    auto lines = scp::getLines(arguments[1]);
    assert(lines.size() == 5);
    CPU cpu = CPU::parse(lines);

    std::vector<int> program;
    for (char c : lines[4]) {
        if (std::isdigit(c)) {
            program.push_back(c - '0');
        }
    }

    auto output = cpu.execute(program);

    std::cout << "part1 answer: " << output << std::endl;

    cpu = CPU::parse(lines);
    constexpr int ntries = 200'000'000;
    for (int i = 0; i < ntries; ++i) {
        CPU copy = cpu;
        copy.regA = i;
        auto output = copy.execute(program);
        if (output == program) {
            std::cout << "part2 answer: " << i << std::endl;
            return 0;
        }
    }
    std::cout << "couldn't find part2 answer after " << ntries << " tries :(" << std::endl;
}
