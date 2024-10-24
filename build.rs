//! Originally from: <https://github.com/rfuzzo/red4lib>

use cmake::Config;
use std::path::Path;

const TARGET_NAME: &str = "kraken_static";

fn main() {
    let kraken_path = Path::new("ext").join("kraken");
    let mut cfg = Config::new(kraken_path);
    let dst = cfg.build_target(TARGET_NAME).build();

    let cmake_profile: String = cfg.get_profile().to_owned();

    let mut link_path = format!("{}/build/bin/CMake", dst.display());
    let mut additional_args = "".to_owned();
    if cfg!(windows) {
        link_path = format!("{}/{}", link_path, cmake_profile);
    } else if cfg!(unix) {
        "-l".clone_into(&mut additional_args);
    }

    println!("cargo:rustc-link-search=native={}", link_path);
    println!(
        "cargo:rustc-link-lib{}=static={}",
        additional_args, TARGET_NAME
    );
}
