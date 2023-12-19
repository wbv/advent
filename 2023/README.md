Advent of Code 2023
-------------------

**NOTE: SPOILERS FOR THE 2023 ADVENT OF CODE PUZZLES**

This is my attempt at doing some of the
[2023 Advent of Code challenges](https://adventofcode.com/events/2023).

My implementation is contained as a singular library crate, with each day's
puzzle solutions being generated from a separate module, with answers (and
examples) solved as integration tests which can be automatically run with cargo:

```sh
cargo test
```

Individual days can be run by specifying a `--test`, for example:

```sh
# verifies day 5's solutions
cargo test --test day05
```

Puzzle inputs are stored in the `/inputs` folder in plain text, and tests are
configured to read the appropriate one from the structure of the source code
tree.
