# Advent of Code 2023

Advent of Code 2023 with 🦀 Rust (1.74.0).

To test that all the solvers are working for the challenge example and my given input:

```bash
cargo test
```

To apply a solver on your own input, run the solver binary with the path to your input file, e.g.:

```bash
cargo run --release --quiet --bin day01-part1 day01/examples/part1-example.txt
cargo run --release --quiet --bin day01-part2 day01/examples/part2-example.txt
```

Each day has a README with the challenge description, example inputs in `examples`
and the solver code is usually in `dayXX/src/lib.rs`.

## 📝 Development

To start a new day of challenges, base it on day00:

```bash
NEW_DAY=day02 ./rise-and-shine.sh
```

## 💡 Preface

I wanted to keep the base dependencies to a minimum; normally I would've used stuff like
`anyhow` to simplify the `?` error handling and `clap` to create the CLI. But I'm sure
I need to use a few common crates for specific challenges, though. 🤏

This didn't really need to have each challenge solver (each day has two challenges) as its
own executable, but I wanted to try out the Rust workspaces and how smooth library code
sharing is within them. 🤔

With this setup, I can send each solver binary to the right Elf for them to run
on their own input. ♻️

Separate binaries also remove the need for having command line arguments except for
the input file path. I hear Elves are notoriously bad at writing command line arguments. 🎅

But, I mean, it _does create a separate binary for each solver_, so they will take space, like
5MB - 50MB each depending on which dependencies are used and if it's a debug or release build. 🤷
