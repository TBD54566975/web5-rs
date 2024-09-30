use errors::Result;
use tokio::runtime::Runtime;
use web5::errors::Web5Error;

pub mod credentials;
pub mod crypto;
pub mod dids;

pub mod errors;

pub fn get_rt() -> Result<Runtime> {
    let rt = Runtime::new()
        .map_err(|e| Web5Error::Unknown(format!("unable to instantiate tokio runtime {}", e)))?;
    Ok(rt)
}
