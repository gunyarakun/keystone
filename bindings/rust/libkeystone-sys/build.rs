extern crate os_type;
extern crate build_helper;

use std::env;
use std::process::Command;

use build_helper::rustc::{link_search, link_lib};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut make_args = vec!["lib_only"];
    if let os_type::OSType::OSX = os_type::current_platform().os_type {
        make_args.push("macos-no-universal");
    }

    let _ = Command::new("mkdir build")
        .current_dir("../../..")
        .arg("build")
        .status();

    let _ = Command::new("./make-lib.sh")
        .current_dir("../../../build")
        .args(&make_args)
        .status();

    let keystone = "llvm/lib/libkeystone.a";
    let _ = Command::new("cp")
        .current_dir("../../../build")
        .arg(&keystone)
        .arg(&out_dir)
        .status();

    link_search(
        Some(build_helper::SearchKind::Native),
        build_helper::out_dir(),
    );
    link_lib(Some(build_helper::LibKind::Static), "keystone");
}
