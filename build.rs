extern crate bindgen;

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("failed to get envvar OUT_DIR");

    let emcc_output = Command::new("emcc")
        .arg("--cflags")
        .output()
        .expect("failed to execute process");
    if !emcc_output.status.success() {
        panic!("failed to execute command: {}",
               String::from_utf8_lossy(&emcc_output.stderr));
    }

    let clang_args = String::from_utf8_lossy(&emcc_output.stdout);

    let whitelist = "^_?em_|^_?emscripten_|^_?EM_|^_?EMSCRIPTEN_";

    let mut builder = bindgen::builder()
        .header("etc/emscripten.h")
        .generate_comments(true)
        .whitelisted_type(whitelist)
        .whitelisted_function(whitelist)
        .whitelisted_var(whitelist)
        .no_unstable_rust()
        .use_core();

    for arg in clang_args.split_whitespace() {
        builder = builder.clang_arg(arg);
    }

    builder.generate()
        .expect("failed to generate rust bindings")
        .write_to_file(Path::new(&out_dir).join("emscripten.rs"))
        .expect("failed to write to file")
}
