<img src="./.assets/christmas_ferris.png" width="164">

# ๐ Advent of Code 2022

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2022 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2022/day/1) | โญ | โญ |
| [Day 2](https://adventofcode.com/2022/day/2) | โญ | โญ |
| [Day 3](https://adventofcode.com/2022/day/3) | โญ | โญ |
| [Day 4](https://adventofcode.com/2022/day/4) | โญ | โญ |
| [Day 5](https://adventofcode.com/2022/day/5) | โญ | โญ |
| [Day 6](https://adventofcode.com/2022/day/6) | โญ | โญ |
| [Day 7](https://adventofcode.com/2022/day/7) | โญ | โญ |
<!--- advent_readme_stars table --->

---

## Template setup

This repo is based on [this template](https://github.com/fspoettel/advent-of-code-rust), for instructions on setting this up for yourself refer to that repo.


## Usage

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Running `target/debug/01`
# ๐ Part 1 ๐
#
# 6 (elapsed: 37.03ยตs)
#
# ๐ Part 2 ๐
#
# 9 (elapsed: 33.18ยตs)
```

`solve` is an alias for `cargo run --bin`. To run an optimized version for benchmarking, append the `--release` flag.

Displayed _timings_ show the raw execution time of your solution without overhead (e.g. file reads).

### Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# ๐ Part 1 ๐
#
# 0 (elapsed: 170.00ยตs)
#
# ๐ Part 2 ๐
#
# 0 (elapsed: 30.00ยตs)
# <...other days...>
# Total: 0.20ms
```
