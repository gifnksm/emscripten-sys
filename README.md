# Emscripten API bindings for Rust

[![crates.io](http://meritbadge.herokuapp.com/emscripten-sys)](https://crates.io/crates/emscripten-sys)

## How to build

```
. <path to emsdk>/emsdk_env.sh
cargo build
```

## How to test

```
cargo test --target asmjs-unknown-emscripten --no-run
node target/asmjs-unknown-emscripten/debug/emscripten_sys-*.js
```
