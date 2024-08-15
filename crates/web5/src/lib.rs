pub mod credentials;
pub mod crypto;
pub mod dids;

#[cfg(test)]
mod test_helpers;

lazy_static::lazy_static! {
  pub(crate) static ref LOG_LEVEL: Option<String> = {
      std::env::var("WEB5_SDK_LOG_LEVEL").ok()
  };
}

pub(crate) mod logging {
  #[macro_export]
  macro_rules! log_dbg {
      ($msg:expr, $($arg:tt)*) => {
          if let Some(ref level) = *$crate::LOG_LEVEL {
              if level == "DEBUG" {
                  println!("[DEBUG] {}:{}", env!("GIT_COMMIT_HASH"), format!($msg, $($arg)*));
              }
          }
      };
      ($closure:expr) => {
          if let Some(ref level) = *$crate::LOG_LEVEL {
              if level == "DEBUG" {
                  let msg = $closure();
                  println!("[DEBUG] {}:{}", env!("GIT_COMMIT_HASH"), msg);
              }
          }
      };
  }
}
