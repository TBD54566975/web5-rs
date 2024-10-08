use chrono::{DateTime, Utc};
use clap::Subcommand;
use std::time::SystemTime;
use web5::{
    credentials::{
        CredentialSubject, Issuer, VerifiableCredential, VerifiableCredentialCreateOptions,
    },
    dids::{bearer_did::BearerDid, portable_did::PortableDid},
    json::{FromJson, ToJson},
};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Creates a VC.
    ///
    /// By default, this command creates an unsigned VC in JSON format.
    /// If a Portable DID is provided, this command outputs a signed VC
    /// in JWT format.
    ///
    /// Examples:
    ///
    /// Create a VC with an issuer and return the minified form
    ///
    /// $ web5 vc create did:dht:36xw3konj1pdd93axsn9p3a58a83uatcgx1nsjud97d91dtr56ry \
    ///     --issuer did:dht:36xw3konj1pdd93axsn9p3a58a83uatcgx1nsjud97d91dtr56ry \
    ///     --no-indent
    ///
    /// Create a VC with a portable DID and return escaped JSON
    ///
    /// $ web5 vc create did:dht:36xw3konj1pdd93axsn9p3a58a83uatcgx1nsjud97d91dtr56ry \
    ///     --portable-did '{"uri": ... }' \
    ///     --json-escape
    #[command(verbatim_doc_comment)]
    Create {
        /// The DID of the entity that the credential is being issued to.
        credential_subject_id: String,
        /// The DID used to sign the VC. If included, the Portable DID's
        /// URI is automatically set as the issuer, and the VC is signed.
        #[arg(short, long)]
        portable_did: Option<String>,
        /// The DID of the issuer of the credential. Required if --portable-did
        /// is not given. Overrides the issuer of the Portable DID if both are passed.
        #[arg(short, long)]
        issuer: Option<String>,
        /// The date when the credential expires (in ISO 8601 standard format).
        /// If not specified, the VC will not expire
        #[arg(long)]
        expiration_date: Option<String>,
        /// If true, output will be minified
        #[arg(long)]
        no_indent: bool,
        /// If true, output JSON will be escaped
        #[arg(long)]
        json_escape: bool,
    },
    Verify {
        vc_jwt: String,
        #[arg(long)]
        no_indent: bool,
        #[arg(long)]
        json_escape: bool,
    },
}

impl Commands {
    pub async fn command(&self) {
        match self {
            Commands::Create {
                credential_subject_id,
                portable_did,
                issuer,
                expiration_date,
                no_indent,
                json_escape,
            } => {
                let portable_did = portable_did
                    .as_ref()
                    .map(|p| PortableDid::from_json_string(p).unwrap());
                let issuer = Issuer::String(match issuer {
                    Some(i) => i.to_string(),
                    None => match &portable_did {
                        Some(p) => p.did_uri.to_string(),
                        None => panic!("either --issuer or --portable-did must be supplied"),
                    },
                });
                let expiration_date = match expiration_date {
                    None => None,
                    Some(d) => match d.parse::<DateTime<Utc>>() {
                        Ok(datetime) => Some(SystemTime::from(datetime)),
                        Err(e) => {
                            panic!("Error parsing date string: {}", e);
                        }
                    },
                };

                let vc = VerifiableCredential::create(
                    issuer,
                    CredentialSubject {
                        id: credential_subject_id.to_string(),
                        ..Default::default()
                    },
                    Some(VerifiableCredentialCreateOptions {
                        expiration_date,
                        ..Default::default()
                    }),
                )
                .await
                .unwrap();

                let mut output_str = match no_indent {
                    true => vc.to_json_string().unwrap(),
                    false => serde_json::to_string_pretty(&vc).unwrap(),
                };

                if *json_escape {
                    output_str = output_str.replace('"', "\\\"");
                }

                println!("{}", output_str);

                if let Some(portable_did) = portable_did {
                    let bearer_did = BearerDid::from_portable_did(portable_did).unwrap();
                    let vc_jwt = vc.sign(&bearer_did, None).unwrap();
                    println!("\n{}", vc_jwt);
                }
            }
            Commands::Verify {
                vc_jwt,
                no_indent,
                json_escape,
            } => match VerifiableCredential::from_vc_jwt(vc_jwt, true).await {
                Err(e) => {
                    println!("\n❌ Verfication failed\n");
                    println!("{:?} {}", e, e);
                }
                Ok(vc) => {
                    println!("\n✅ Verfied\n");

                    let mut output_str = match no_indent {
                        true => serde_json::to_string(&vc).unwrap(),
                        false => serde_json::to_string_pretty(&vc).unwrap(),
                    };

                    if *json_escape {
                        output_str = output_str.replace('"', "\\\"");
                    }

                    println!("{}", output_str);
                }
            },
        }
    }
}
