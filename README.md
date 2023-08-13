# stressed &mdash; a CLI tool to stress-test solutions in competitive programming

Sad, but often when solving competitive programming problems they fail, and
on a platform that doesn't let you see the test cases that can be a problem.
However, often it's easy to test the solution on small testcases, where another
(probably more naive and simple) solution is feasible. Than by combining *sampler*,
which generates small test cases randomly,
and reference *solver*, broadly called *checker*, we can evaluate our solution and
fix it.
This project is aimed at providing flexible and **fast** tool to perform such testing.

## Features
- Does not interrupt the usual workflow: you only have to supply sampler, which outputs samples to stdout, and reference solver,
    which is of the same format as the solution. There is no need to modify your solution in any way.
- Fast: it uses asynchronous process spawns and outperforms naive realization by 2-5 times
- Can use random seeds for sampler to facilitate reproducible testing
- Customizable:
    - Use args/stdin to supply random seeds to sampler (which can ignore them whatsoever)
    - Use default checker, which compares output with the reference solver, or use dedicated checker, which can
        check the output in any way
    - Show diffs with the correct solution (character-wise/line-wise) or do not show them at all
    - Control the number of iterations
    - Show progress bar

For details on usage see the [Usage](#usage) section.

## Installation
You can either compile from source using *cargo* or download the precompiled release files at Github.
To install via cargo run
```rust
    cargo install stressed
```
Of course, you need to have Rust toolchain, including *cargo*, to do so.
The easier alternative is to use precompiled statically-linked binaries under the releases tab on Github.
Those are built using Github Actions and you can inspect the build scripts yourself, so it's safe.

## Usage
The CLI documentation can be found [here](docs/CLI.md)
