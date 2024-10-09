use std::env;

// ANSI color codes
const YELLOW_COLOR: &str = "\x1b[93m";
const RESET_COLOR: &str = "\x1b[0m";

// Function to check if the current process has root privileges
pub fn is_root() -> bool {
    if let Ok(user) = env::var("USER") {
        user == "root"
    } else {
        false
    }
}

// Function to display a warning if not running as root
pub fn warn_if_not_root() {
    const WARNING_MESSAGE: &str =
        "Warning: This command may require root privileges to function properly.";

    if !is_root() {
        eprintln!("\n{}{}{}\n", YELLOW_COLOR, WARNING_MESSAGE, RESET_COLOR);
    }
}
