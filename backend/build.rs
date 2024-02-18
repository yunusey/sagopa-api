//! While writing this file, most of the things were copied from
//! [shuttle-fullstack-rust-example](https://github.com/TylerBloom/shuttle-fullstack-rust-example/blob/main/backend/build.rs).
//! Thanks for the support!

use std::path::Path;
use std::{env, process::Command};

fn main() -> Result<(), i32> {
    let wd = env::var("CARGO_MANIFEST_DIR").unwrap();
    let backend_path = format!("{wd}");
    let root_path = Path::new(&backend_path).parent().unwrap();
    let frontend_path = root_path.join("frontend");

    // Install external dependency (in the shuttle container only)
    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle")
    {
        // Install the `wasm32-unknown-unknown` target
        if !std::process::Command::new("rustup")
            .args(["target", "add", "wasm32-unknown-unknown"])
            .status()
            .expect("failed to run rustup")
            .success()
        {
            panic!("failed to install rustup")
        }

        // Install `trunk` to compile the frontend
        if !std::process::Command::new("cargo")
            .args(["install", "trunk"])
            .status()
            .expect("failed to run rustup")
            .success()
        {
            panic!("failed to install rustup")
        }
    }

    let assets_path = root_path.join("assets").to_str().unwrap().to_owned();
    let path_to_html = frontend_path
        .join("index.html")
        .to_str()
        .unwrap()
        .to_owned();

    let mut cmd = Command::new("trunk");
    cmd.args([
        "build",
        "-d",
        &assets_path,
        "--filehash",
        "false",
        &path_to_html,
    ]);
    if Ok("release".to_owned()) == env::var("PROFILE") {
        cmd.arg("--release");
    }
    cmd.status().map(|s| s.success()).unwrap_or_else(|_| false);
    // match cmd.status().map(|s| s.success()) {
    //     Ok(false) | Err(_) => return Err(1),
    //     _ => {}
    // }
    // // println!("cargo:rerun-if-changed={fe_path}");
    // // println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
