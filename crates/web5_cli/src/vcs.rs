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
        /// The optional credential status, which may indicate revocation or suspension information.
        #[arg(long)]
        credential_status: Option<String>,
        /// The credential schema, used to validate the data structure of the credential. This is optional.
        /// JSON Schema validation is performed if the value is provided, and creation will fail if validation fails.
        #[arg(long)]
        credential_schema: Option<String>,
        /// An optional array of evidence supporting the claims made in the credential.
        #[arg(long)]
        evidence: Option<String>,
        /// The type(s) of the Verifiable Credential.
        #[arg(long)]
        r#type: Option<Vec<String>>,
        /// The context(s) for the Verifiable Credential, which define the meaning of terms within the credential.
        #[arg(long)]
        context: Option<Vec<String>>,
        /// The unique identifier for the Verifiable Credential. This is optional.
        #[arg(long)]
        id: Option<String>,
        /// The issuance date of the credential. If not provided, defaults to the current date and time.
        #[arg(long)]
        issuance_date: Option<String>,
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
                credential_status,
                credential_schema,
                evidence,
                r#type,
                context,
                id,
                issuance_date,
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
                let issuance_date = match issuance_date {
                    None => None,
                    Some(d) => match d.parse::<DateTime<Utc>>() {
                        Ok(datetime) => Some(SystemTime::from(datetime)),
                        Err(e) => {
                            panic!("Error parsing date string: {}", e);
                        }
                    },
                };
                let credential_status = credential_status.as_ref().map(|cs| {
                    serde_json::from_str(cs).expect("Error parsing credential status JSON string")
                });
                let credential_schema = credential_schema.as_ref().map(|cs| {
                    serde_json::from_str(cs).expect("Error parsing credential schema JSON string")
                });
                let evidence = evidence.as_ref().map(|e| {
                    serde_json::from_str(e).expect("Error parsing evidence JSON string")
                });

                let vc = VerifiableCredential::create(
                    issuer,
                    CredentialSubject {
                        id: credential_subject_id.to_string(),
                        ..Default::default()
                    },
                    Some(VerifiableCredentialCreateOptions {
                        id: id.clone(),
                        context: context.clone(),
                        r#type: r#type.clone(),
                        issuance_date,
                        expiration_date,
                        credential_status,
                        credential_schema,
                        evidence,
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
