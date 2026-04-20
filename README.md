# concise_vec

[![Crates.io](https://img.shields.io/crates/v/concise_vec.svg)](https://crates.io/crates/concise_vec)
[![Docs.rs](https://docs.rs/concise_vec/badge.svg)](https://docs.rs/concise_vec)
[![License](https://img.shields.io/crates/l/concise_vec.svg)](https://github.com/paladynee/concise_vec#license)

A concise, highly optimized vector implementation leveraging `const generics` and advanced compile-time evaluation features in Rust. 

## Experimental Notice
**Note:** `concise_vec` heavily utilizes experimental Rust nightly features (such as `generic_const_exprs`, `const_trait_impl`, `const_slice_make_iter`, etc.) to achieve its compile-time guarantees and zero-cost abstractions. You will need a **nightly Rust compiler** to build this crate.

## Features
- **Const Generics & Metaprogramming:** Moves many structural and algebraic checks to compile time.
- **`no_std` Compatible:** Designed with minimalistic environments in mind, featuring configurable `std` and `alloc` features.
- High layout optimization using custom uninitialized strategies and field alignments.

## Installation

Run this command in your project root:

```bash
cargo add concise_vec
```

Because of the heavy reliance on nightly features, ensure you run your project with a nightly toolchain:

```bash
rustup override set nightly
```

## License

This project is dual-licensed under either the [MIT license](LICENSE-MIT) or the [Apache License, Version 2.0](LICENSE-APACHE), at your option.
