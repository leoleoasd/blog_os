// build.rs
use std::process::Command;
fn main() {
    // note: add error checking yourself.
    let hash = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let git_hash = String::from_utf8(hash.stdout).unwrap();

    let tag = Command::new("git").args(&["describe", "--tags"]).output().unwrap();
    let git_tag = String::from_utf8(tag.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=GIT_TAG={}", git_tag);
}
