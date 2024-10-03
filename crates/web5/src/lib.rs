pub mod credentials;
pub mod crypto;
pub mod dids;

mod datetime;
pub mod errors;
mod http;
pub mod jose;
pub mod json;

pub use http::set_http_client;

#[cfg(test)]
mod test_vectors;
#[cfg(test)]
mod tests {
    #[cfg(feature = "http_reqwest")]
    #[test]
    fn test_with_reqwest_feature() {
        println!("http_reqwest feature is enabled!");
    }

    #[cfg(not(feature = "http_reqwest"))]
    #[test]
    fn test_without_reqwest_feature() {
        println!("http_reqwest feature is NOT enabled!");
    }
}
