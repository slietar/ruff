# no_std parser crates

This branch patches the crates that `ruff_python_parser` needs so that they
build with `#![no_std]` and `alloc` only, for use by
[Rhûn](https://github.com/slietar). It is based on Ruff 0.17.5.

Patched crates: `ruff_python_parser`, `ruff_python_ast`, `ruff_python_trivia`,
`ruff_source_file`, `ruff_text_size` and `ruff_unicode_names2`. Only these
crates are expected to build on this branch: the rest of the workspace relies
on features that were removed from them.

## What changed

- Every patched crate is `#![no_std]` with `extern crate alloc`, and every
  module imports the `alloc` types that the std prelude used to provide
  (`String`, `Vec`, `Box`, `ToString`, `ToOwned`, `format!`, `vec!`).
- `std::` paths point to `core::` or `alloc::`.
- The patched crates no longer inherit workspace dependencies. They list
  explicit versions with `std` features turned off.
- `FxHashMap` / `FxHashSet` (which need `std`) are `hashbrown` maps with
  `FxBuildHasher`.
- `LazyLock` / `OnceLock` are replaced with `spin::Lazy` and a small `OnceLock`
  wrapper around `spin::Once`.
- In `ruff_python_ast`, the APIs that take a `Path` (`SourceType::from`,
  `PySourceType::from` / `try_from_path`, `helpers::to_module_path`) are behind
  a new `std` feature, and `extern crate core as std` lets the `is_macro`
  derive, which emits `::std::` paths, resolve against `core`.
- `ruff_python_parser` no longer forces the `get-size` feature. It is now an
  optional feature of the parser.
- `drop_bomb::DebugDropBomb` (which needs `std::thread::panicking`) is replaced
  with a no-op stand-in, so the parser loses one debug-only consistency check.
- The `serde`, `salsa`, `schemars` and `cache` features and the dev-dependencies
  of the patched crates were removed.

## Updating

Rebase this branch onto a newer release tag. New `std` uses show up as errors
when building for a target without `std`, for example
`cargo build -p ruff_python_parser --target thumbv7em-none-eabihf`.
