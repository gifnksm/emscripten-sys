# Emscripten API bindings for Rust

[![Build Status](https://travis-ci.org/gifnksm/emscripten-sys.svg?branch=master)](https://travis-ci.org/gifnksm/emscripten-sys)
[![crates.io](http://meritbadge.herokuapp.com/emscripten-sys)](https://crates.io/crates/emscripten-sys)

[Documentation](https://gifnksm.github.io/emscripten-sys/emscripten_sys)

## Usage

Add the following dependencies to your `Cargo.toml`:

```toml
[target.'cfg(target_os = "emscripten")'.dependencies]
emscripten-sys = "0.3"
```

## How to build

```
. <path to emsdk>/emsdk_env.sh
cargo build --target asmjs-unknown-emscripten
```

## How to test

```
cargo test --target asmjs-unknown-emscripten --no-run
node target/asmjs-unknown-emscripten/debug/emscripten_sys-*.js
```
