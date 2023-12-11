// use crate::error::Result;
// use crate::key::private_key::PrivateKeyFfi;
// use crypto::key_manager::key_store::{
//     KeyStore as CryptoKeyStore, KeyStoreError as CryptoKeyStoreError,
// };
// use std::sync::Arc;
//
// pub trait CustomKeyStore: Send + Sync {
//     fn get(&self, key_alias: String) -> Result<Option<Arc<PrivateKeyFfi>>>;
//     fn insert(&self, key_alias: String, private_key: Arc<PrivateKeyFfi>) -> Result<()>;
// }
//
// // pub(crate) struct CustomKeyStoreAdapter(pub(crate) Arc<dyn CustomKeyStore>);
// //
// // impl CryptoKeyStore for CustomKeyStoreAdapter {
// //     fn get(&self, key_alias: &str) -> Result<Option<CryptoPrivateKey>, CryptoKeyStoreError> {
// //         let private_key = self.0.get(key_alias.to_string())?;
// //         Ok(private_key.map(|k| k.0.clone()))
// //     }
// //
// //     fn insert(
// //         &self,
// //         key_alias: &str,
// //         private_key: CryptoPrivateKey,
// //     ) -> Result<(), CryptoKeyStoreError> {
// //         Ok(self.0.insert(
// //             key_alias.to_string(),
// //             Arc::new(PrivateKey::from(private_key)),
// //         )?)
// //     }
// // }
