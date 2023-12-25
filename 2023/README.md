Advent of Code 2023
-------------------

**NOTE: SPOILERS FOR THE 2023 ADVENT OF CODE PUZZLES**

This is my attempt at doing some of the
[2023 Advent of Code challenges](https://adventofcode.com/events/2023).

My implementation is contained as a singular library crate, with each day's
puzzle solutions computed via unit tests, with answers (and examples) solved as
assertions in the unit tests.  Control of which test(s) is handled with run with
the standard cargo test controls. So:

```sh
cargo test
```

Runs all tests/solutions from all days.

Individual days or tests can be run by specifying a substring pattern to match.
For example:

```sh
# verify day 5's solutions, day05::part1 and day05::part2
cargo test day05::part

# run all of the part1's (for all days)
cargo test ::part1

# run the examples for all days
cargo test ::ex
```

Puzzle inputs are stored in the `/inputs` folder in plain text, and tests are
configured to read the appropriate one from the structure of the source code
tree.
