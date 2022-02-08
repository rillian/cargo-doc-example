# Issue demonstration

[![cargo build](https://github.com/rillian/cargo-doc-example/actions/workflows/build.yml/badge.svg)](https://github.com/rillian/cargo-doc-example/actions/workflows/build.yml)
[![cargo doc](https://github.com/rillian/cargo-doc-example/actions/workflows/doc.yml/badge.svg)](https://github.com/rillian/cargo-doc-example/actions/workflows/doc.yml)

This is a demonstration of [cargo issue #10373](https://github.com/rust-lang/cargo/issues/10373) where `cargo doc --example foo` doesn't pass the crates from `dev-dependencies` like `cargo run --example foo` does. If both of the CI badges above are passing, the issue should be fixed.

# Steps to reproduce

```sh
git clone https://github.com/rillian/cargo-doc-example
cd cargo-doc-example
cargo run --example main
cargo doc --example main
```

As of cargo 1.58, the last command fails with

```
error[E0432]: unresolved import `env_logger`
 --> examples/main.rs:2:5
  |
2 | use env_logger::Env;
  |     ^^^^^^^^^^ use of undeclared crate or module `env_logger`

error: Compilation failed, aborting rustdoc
```
