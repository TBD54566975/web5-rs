use chrono::{DateTime, Utc};
use clap::Subcommand;
use std::time::SystemTime;
use uuid::Uuid;
use web5::{
    credentials::verifiable_credential_1_1::{
        CredentialSubject, Issuer, VerifiableCredential, BASE_CONTEXT, BASE_TYPE,
    },
    dids::{bearer_did::BearerDid, portable_did::PortableDid},
};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Create {
        credential_subject_id: String,
        #[arg(long, help = "If provided, the VC will be signed")]
        portable_did: Option<String>,
        #[arg(long)]
        issuer: Option<String>,
        #[arg(long)]
        expiration_date: Option<String>,
        #[arg(long)]
        no_indent: bool,
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
    pub fn command(&self) {
        match self {
            Commands::Create {
                credential_subject_id,
                portable_did,
                issuer,
                expiration_date,
                no_indent,
                json_escape,
            } => {
                let portable_did = portable_did.as_ref().map(|p| PortableDid::new(p).unwrap());
                let issuer = Issuer::String(match issuer {
                    Some(i) => i.to_string(),
                    None => match &portable_did {
                        Some(p) => p.did_uri.to_string(),
                        None => String::default(),
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

                let now = SystemTime::now();
                let vc = VerifiableCredential::create(
                    format!("urn:vc:uuid:{0}", Uuid::new_v4()),
                    vec![BASE_CONTEXT.to_string()],
                    vec![BASE_TYPE.to_string()],
                    issuer,
                    now,
                    expiration_date,
                    CredentialSubject {
                        id: credential_subject_id.to_string(),
                        ..Default::default()
                    },
                );

                let mut output_str = match no_indent {
                    true => serde_json::to_string(&vc).unwrap(),
                    false => serde_json::to_string_pretty(&vc).unwrap(),
                };

                if *json_escape {
                    output_str = output_str.replace('"', "\\\"");
                }

                println!("{}", output_str);

                if let Some(portable_did) = portable_did {
                    let bearer_did = BearerDid::from_portable_did(portable_did).unwrap();
                    let vc_jwt = vc.sign(&bearer_did).unwrap();
                    println!("\n{}", vc_jwt);
                }
            }
            Commands::Verify {
                vc_jwt,
                no_indent,
                json_escape,
            } => match VerifiableCredential::verify(vc_jwt) {
                Err(e) => {
                    println!("\n❌ Verfication failed\n");
                    println!("{:?} {}", e, e);
                }
                Ok(_) => {
                    let vc = VerifiableCredential::verify(vc_jwt).unwrap();
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
