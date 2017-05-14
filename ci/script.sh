#!/bin/bash

set -ev

emcc --version
rustc -vV
cargo -vV

TARGET=asmjs-unknown-emscripten

cargo build --target ${TARGET}
cargo test --target ${TARGET} --no-run
#node target/${TARGET}/debug/emscripten_sys-*.js
cargo doc --target ${TARGET}
