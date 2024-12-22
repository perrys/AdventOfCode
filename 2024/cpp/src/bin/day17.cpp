#include "lib/file_utils.hpp"
#include "lib/transform.hpp"

#include <assert.h>

#include <algorithm>
#include <cmath>
#include <cstdint>
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

template <typename T> std::ostream& operator<<(std::ostream& ostream, const std::vector<T>& vals) {
    bool first = true;
    for (auto i : vals) {
        if (first) {
            first = false;
        } else {
            ostream << ",";
        }
        ostream << (size_t)i;
    }
    return ostream;
}

namespace {

struct CPU {
    size_t regA;
    size_t regB;
    size_t regC;
    size_t pc;

    static auto parse(std::vector<std::string>& lines) -> CPU {
        // auto parser = [](const std::string& line) -> int {
        //     auto range = line                                                                 //
        //                  | std::ranges::views::filter([](auto c) { return std::isdigit(c); }) //
        //                  | std::ranges::views::transform(scp::parseInt());
        //     return *(range.begin());
        // };
        auto parser = [](const std::string& line) -> size_t {
            std::vector<char> digits;
            for (auto c : line) {
                if (std::isdigit(c)) {
                    digits.push_back(c);
                }
            }
            const char* start = &digits[0];
            return scp::Parser<size_t>::parse(start, start + digits.size()).value();
        };
        return {parser(lines[0]), parser(lines[1]), parser(lines[2]), 0};
    }

    size_t comboOperand(size_t val) {
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

    auto execute(const std::vector<uint8_t>& program) -> std::vector<uint8_t> {
        const bool debug = false;
        if (debug) {
            std::cout << "CPU: ";
            this->output(std::cout);
        }
        std::vector<uint8_t> output;

        while (this->pc < program.size()) {
            auto opcode = program.at(this->pc);
            auto operand = program.at(this->pc + 1);
            assert(opcode >= 0 && opcode <= 7);
            assert(operand >= 0 && operand <= 7);
            if (debug) {
                std::cout << "opcode: " << (size_t)opcode << ", operand: " << (size_t)operand
                          << std::endl;
            }
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
                if (debug) {
                    std::cout << "output x: " << comboOperand(operand) % 8 << std::endl;
                }
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
                std::cout << "CPU: ";
                this->output(std::cout);
            }
        }
        return output;
    }
};

std::vector<size_t> recurse(size_t depth, const std::vector<uint8_t>& program,
                            const CPU& initialState) {

    bool debug = false;
    if (depth == 1) {
        return {1, 2, 3, 4, 5, 6, 7};
    }
    std::vector<size_t> candidates = recurse(depth - 1, program, initialState);
    std::vector<uint8_t> test(program.begin() + program.size() - depth + 1, program.end());
    std::vector<size_t> result;
    for (auto val : candidates) {
        CPU cpu = initialState;
        cpu.regA = val;
        std::vector<uint8_t> output = cpu.execute(program);
        if (debug) {
            std::cout << "depth: " << depth << ", regA: " << val << "\nout: " << output
                      << "\ntest: " << test << "\n"
                      << std::endl;
        }
        if (output == test) {
            for (size_t i = val * 8; i < (val + 1) * 8; ++i) {
                result.push_back(i);
            }
        }
    }
    if (result.size() == 0) {
        throw std::runtime_error("unable to find result at depth " + std::to_string(depth));
    }
    return result;
}

} // namespace

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

    std::vector<uint8_t> program;
    for (char c : lines[4]) {
        if (std::isdigit(c)) {
            program.push_back(c - '0');
        }
    }

    auto output = cpu.execute(program);
    std::cout << "part1 answer: " << output << std::endl;

    /* The disassembly of my input data is:
start:
        2,4 bst %rA     # let %rB = %rA % 8
        1,7 bxl $7      # let %rB = %rB xor 0111b
        7,5 cdv %rB     # let %rC = %rA / 2^%B
        1,7 bxl $7      # let %rB = %rB xor 0111b
        4,6 bxc         # let %rB = %rB xor %rC
        0,3 adv $3      # let %rA = %rA / 8
        5,5 out %rA     # print %rB and 0111b
        3,0 jnz $0      # if %rA !=0 goto start:

        The important thing to notice is that register A gets shifted right by 3 bits each
        iteration, and there are 16 iterations so we must start with a number ~ 2^48, or 10^14. So
        it becomes clear why brute force searching won't work :). The example input also includes
        "adv $3" so I guess this is common to all inputs.

        This is a recursive problem, working backwards from the end, for each iteration we need to
        find the set of numbers which when we shift them 3 bits right gives the next output. There
        are 8 possibilities for each iteration, because for integer division, there are a number of
        values which can procuce the same answer.

        e.g. 229 // 8 = 28, lower bound = 8 * 28 = 224, upper bound = 8 * (1 + 28) - 1 = 231
     */

    CPU initialState = CPU::parse(lines);
    try {
        std::vector<size_t> candidates = recurse(program.size(), program, initialState);
        std::vector<size_t> answers;
        for (auto val : candidates) {
            CPU cpu = initialState;
            cpu.regA = val;
            auto output = cpu.execute(program);
            if (output == program) {
                answers.push_back(val);
            }
        }
        std::ranges::sort(answers);
        std::cout << "part2 answer " << answers.at(0) << std::endl;
    } catch (std::exception& e) {
        std::cerr << e.what() << std::endl;
    }
}
