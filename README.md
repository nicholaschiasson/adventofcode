# adventofcode
My [Advent of Code](https://adventofcode.com) solutions.

## Prerequisites

Let's go over what you'll need...

- [nix]
- [nix flakes]

That's really it.

## Local Development

Before doing anything, you need to build the development shell:

```
nix develop
```

That will expose all the tooling you need. From there, you can use justfile
to operate the repo.

The default task when you simply run `just` is `just test-all`.

See available tasks defined in justfile:

```
just --list
```

### Doing an Advent of Code challenge

You can perform most operations in the justfile for all languages (as the tasks
will usually have a `{task}-all` counterpart), but typically we will focus on
one language at a time. For this example, we'll start a new challenge using
rust.

The first thing to do is generate the scaffolding for the new challenge:

```
just bootstrap rust 2024 10 2
```

This example will create files for two practice inputs and a final input for
the challenge, as well as a rust source file with the boilerplate to begin the
the challenge for the day.

```
.
├── rsrc
│   └── inputs
│       └── year_2024
│           └── day_10
│               └── tests
│                   ├── final.txt
│                   ├── practice_1.txt
│                   └── practice_2.txt
└── rust
    └── src
        └── year_2024
            └── day_10.rs
```

> [!Note] 
> In the case of rust, the new source file will not yet by included by the
> compiler. You need to manually include it in a module. For this example, that
> module will be at `./rust/src/year_2024/mod.rs`.

```rust
...
pub mod day_10;
```

After that, code away, and test as you make progress:

```
just test rust year_2024::day_10::tests::part_01
```

Better yet, watch for file changes and re-run tests automatically:

```
just watch rust -x "'\"test year_2024::day_10::tests::part_02\"'"
```

> [!NOTE]
> The crazy quotes are necessary here due to the underlying `cargo watch`
> command and how `just` splits arguments.

[nix]: https://nixos.org/download.html
[nix flakes]: https://nixos.wiki/wiki/Flakes#Enable_flakes
