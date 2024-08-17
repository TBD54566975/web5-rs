use std::{env, sync::LazyLock};

pub mod credentials;
pub mod crypto;
pub mod dids;

#[cfg(test)]
mod test_helpers;

pub const GIT_COMMIT_HASH: &str = env!("WEB5_GIT_COMMIT_HASH");

// TODO: https://github.com/TBD54566975/web5-rs/issues/287
#[allow(dead_code)]
static LOG_LEVEL: LazyLock<Option<String>> = LazyLock::new(|| {
    // Default log level if the environment variable is not set
    env::var("LOG_LEVEL").ok()
});

pub(crate) mod logging {
    #[macro_export]
    macro_rules! log_dbg {
        ($msg:literal $(, $arg:tt)*) => {
            if let Some(ref level) = *$crate::LOG_LEVEL {
                if level == "DEBUG" {
                  println!("[DEBUG] {}:{}", env!("WEB5_GIT_COMMIT_HASH"), format!($msg, $($arg)*));
                }
            }
        };
        ($closure:expr) => {
            if let Some(ref level) = *$crate::LOG_LEVEL {
                if level == "DEBUG" {
                    let msg = $closure();
                    println!("[DEBUG] {}:{}", env!("WEB5_GIT_COMMIT_HASH"), msg);
                }
            }
        };
    }
}

#[cfg(test)]
mod test {
    use crate::log_dbg;

    #[test]
    fn can_log_dbg() {
        log_dbg!("Log debugging without arguments");
        log_dbg!("Log debugging with arguments {}", "Some value");
        log_dbg!(|| { 2 + 2 });
    }
}
