use std::{fs, process::Command};

fn main() {
    // Execute the `git rev-parse HEAD` command to get the current commit hash
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("Failed to execute git command");

    // Convert the output to a string
    let git_hash = String::from_utf8(output.stdout).expect("Invalid UTF-8 sequence");

    // Remove the newline character from the commit hash
    let git_hash_trimmed = git_hash.trim();

    let dest_path = format!("src/resources/git_sha.txt");
    fs::write(dest_path, git_hash_trimmed).expect("Unable to write file");

    // Pass the commit hash to the compiler as an environment variable
    println!("cargo:rustc-env=WEB5_GIT_COMMIT_HASH={}", git_hash_trimmed);
}
