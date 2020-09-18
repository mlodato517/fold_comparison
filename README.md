# Fold Comparisons

## Background

This is just me investigating various benchmarks involving `fold()` and `for` loops.
Much of this was kicked off from https://github.com/rust-lang/rust/pull/76746 because
I was confused about the benchmarks being so inconsistent with respect to
https://github.com/rust-lang/rust/issues/76725.

## Results

TBD

## To Run

The non-Criterion benches require nightly rust:

```
rustup toolchain install nightly
rustup default nightly
```

and can be run with

```
cargo bench
```

To view Criterion plots, look in `target/criterion/report/index.html`.
