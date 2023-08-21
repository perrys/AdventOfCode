# Advent of Code Solutions

This repo contains my solutions to some AoC challenges. Currently the
solutions are written (separately) in Rust and x64 assembly. The idea
is to begin each challenge with a fairly straightforward solution in
Rust, and then create a separate solution in assembly, trying to
improve on the performance of the Rust solution. This is done as a
learning exercise and I don't claim that either the Rust or the
assembly solutions are the fastest out there, but hopefully they do
provide some insights into runtime performance improvement. In some
cases, tricks which I have employed in the assembly solution could be
back-ported to the Rust solution to make it run faster, but that was
not really the goal of the exercise &ndash; I trust the Rust compiler
to produce efficient object code even when t is written with
high-level functional concepts, so it provides a baseline performance
measurement to target with the assembly solution. Also, Rust's sandard
library functions tend to do much more input validation etc, so direct
comparison is not fair, and the assembly solutions would generally not
be suitable for production environments without further work.

The solutions below all follow the same pattern - first read the input
file into memory, and then process the buffer repeatedly according to
the rules of the challenge, recording the execution time for each
invocation. The timings below are the lowest wall-clock times from a
group of timings with low standard deviation, measured on my desktop
machine which has an AMD Ryzen 7 7700 CPU.

| Challenge | Rust Solution (ns) | Assembly Solution (ns) |
| :-------- | -----------------: | ---------------------: |
| 2022 Day 1 Part 1 |  9829 |  4899 |
| 2022 Day 1 Part 2 |  9699 |  4989 |
| 2022 Day 2 Part 1 | 43939 | 14479 |
| 2022 Day 2 Part 2 | 43790 | 11670 |
| 2022 Day 3 Part 1 |  3690 |  7629 |
| 2022 Day 3 Part 2 |  5469 |  9729 |


## 2022 Day 1

<https://adventofcode.com/2022/day/1>

This is a fairly straightforward challenge to count numbers in groups
separated by blank lines. 
