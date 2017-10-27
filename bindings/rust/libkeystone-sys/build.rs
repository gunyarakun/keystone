extern crate os_type;
extern crate build_helper;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    run(
        Command::new("mkdir")
            .current_dir("../../..")
            .arg("-p")
            .arg("build"),
    );

    run(Command::new("cmake").current_dir("../../../build").args(
        &[
            &format!("-DCMAKE_INSTALL_PREFIX={}", out_dir.display()),
            "-DCMAKE_BUILD_TYPE=Release",
            "-DBUILD_LIBS_ONLY=1",
            "-DCMAKE_SHARED_LIBS=ON",
            "-DCMAKE_OSX_ARCHITECTURES=",
            "-DLLVM_TARGET_ARCH=host",
            "-G",
            "Unix Makefiles",
            "..",
        ],
    ));

    run(Command::new("make").current_dir("../../../build").arg(
        "install",
    ));

    println!("cargo:rustc-link-search=native={}/lib", out_dir.display());
    println!("cargo:rustc-link-lib=static=keystone");
}

fn run(cmd: &mut Command) {
    println!("run: {:?}", cmd);
    let status = match cmd.status() {
        Ok(s) => s,
        Err(ref e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!(
            "command did not execute successfully, got: {}",
            status
        ));
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s);
}
