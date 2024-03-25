use std::{fs, path::Path, process::Command};

fn spawn_ok(cmd: &mut Command) {
    let code = cmd.status().unwrap();
    assert!(code.success());
}

fn compile_cpp() {
    spawn_ok(Command::new("cmake").args([
        "-S../cpp_tests",
        "-B../target/cpp_build",
        "-DCMAKE_BUILD_TYPE=Release",
    ]));

    spawn_ok(Command::new("cmake").args(["--build", "../target/cpp_build", "--config", "Release"]));

    // TODO: Use the cross platform compilation vars
    let dl_name = if cfg!(target_os = "windows") {
        "cpp_tests.dll"
    } else if cfg!(target_os = "linux") {
        "libcpp_tests.so"
    } else if cfg!(target_os = "macos") {
        "libcpp_tests.dylib"
    } else {
        panic!("what are you running on? 🤔");
    };

    let base_path = Path::new("../target/cpp_build/bin");
    let path1 = base_path.join(dl_name);
    let path2 = base_path.join("Release").join(dl_name);
    let in_path = if path1.exists() {
        &path1
    } else if path2.exists() {
        &path2
    } else {
        panic!(
            "couldn't find cpp dll; tried:\n{}\n{}\n",
            path1.display(),
            path2.display()
        );
    };

    fs::copy(in_path, format!("../target/Debug/{dl_name}")).unwrap();
    fs::copy(in_path, format!("../target/Release/{dl_name}")).unwrap();

    println!("cargo::rerun-if-changed=../cpp_tests");
}

fn generate_header() {
    cbindgen::Builder::new()
        .with_crate("../tests_api")
        .generate()
        .unwrap()
        .write_to_file("../cpp_tests/src/api.hpp");

    println!("cargo::rerun-if-changed=../tests_api");
}

fn main() {
    generate_header();
    compile_cpp();
}
