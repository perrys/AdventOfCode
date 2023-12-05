# AoC 2023

Rust implementations of [Advent of Code 2023](https://adventofcode.com/2023) challenges.

## Test Driven Development Challenge

My firm is holding a TDD challenge using this year's AoC problems. For those
unfamiliar with Rust -- tests are commonly included directly in the source file
the tests apply to, in a sub-module at the end of the file. The `#[cfg(test)]`
attribute means that the test code will not be included in normal builds, only
when running tests (e.g. via `cargo test`).

