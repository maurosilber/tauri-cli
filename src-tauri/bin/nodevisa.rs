use std::{env::current_exe, process::Command};

fn main() {
    let path = current_exe().unwrap().parent().unwrap().join("nv");
    if let Err(e) = Command::new(path.clone()).arg("gui").status() {
        eprintln!("Failed to run {path:?}: {e}");
    }
}
