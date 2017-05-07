#!/bin/bash

set -ev

source /emsdk/emsdk_env.sh

llvm-config --prefix
clang --version
emcc --version
rustc -vV
cargo -vV

TARGET=asmjs-unknown-emscripten

cargo build --target ${TARGET}
cargo test --target ${TARGET} --no-run
#node target/${TARGET}/debug/emscripten_sys-*.js
cargo doc --target ${TARGET}
