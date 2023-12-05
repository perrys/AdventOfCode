# [AoC 2022](https://adventofcode.com/2022)

The idea is to begin each challenge with a fairly straightforward solution in
Rust, and then create a separate solution in assembly, trying to improve on the
performance of the Rust solution. This is done as a learning exercise and I
don't claim that either the Rust or the assembly solutions are the fastest out
there, but hopefully they do provide some insights into runtime performance
improvement. In some cases, tricks which I have employed in the assembly
solution could be back-ported to the Rust solution to make it run faster, but
that was not really the goal of the exercise &ndash; the Rust compiler
produces efficient object code even when it is written with high-level
functional concepts, so it provides a baseline performance measurement to target
with the assembly solution. Also, Rust's sandard library functions tend to do
much more input validation etc, so direct comparison is not fair, and the
assembly solutions would generally not be suitable for production environments
without further work.

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
| 2022 Day 3 Part 1 |  3690 |  3270 |
| 2022 Day 3 Part 2 |  5469 |  4415 |
| 2022 Day 4 Part 1 | 60480 |  7349 |
| 2022 Day 4 Part 2 | 60490 |  7400 |



