#![cfg(feature = "unstable")]

extern crate compiletest_rs as compiletest;

use std::path::PathBuf;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::default_config();
    let cfg_mode = mode.parse().ok().expect("Invalid mode");

    config.mode = cfg_mode;
    config.src_base = PathBuf::from(format!("tests/{}", mode));
    config.target_rustcflags = Some("-L target/debug".to_string());

    compiletest::run_tests(&config);
}

#[test]
fn fail() {
    run_mode("compile-fail");
}
