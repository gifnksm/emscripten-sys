extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("failed to get envvar OUT_DIR");
    let emscripten_dir = env::var("EMSCRIPTEN").expect("failed to get envvar EMSCRIPTEN");

    let whitelist = "^_?em_|^_?emscripten_|^_?EM_|^_?EMSCRIPTEN_";

    let builder = bindgen::builder()
        .header("etc/emscripten.h")
        .generate_comments(true)
        .whitelisted_type(whitelist)
        .whitelisted_function(whitelist)
        .whitelisted_var(whitelist)
        .no_unstable_rust()
        .use_core()
        .clang_arg(format!("-I{}/system/include", emscripten_dir))
        .clang_arg(format!("-I{}/system/include/libc", emscripten_dir))
        .clang_arg(format!("-I{}/system/include/libcxx", emscripten_dir))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .clang_arg("-D__EMSCRIPTEN__");

    builder
        .generate()
        .expect("failed to generate rust bindings")
        .write_to_file(Path::new(&out_dir).join("emscripten.rs"))
        .expect("failed to write to file")
}
