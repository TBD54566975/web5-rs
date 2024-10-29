use clap::Subcommand;

mod create;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Creates a Presentation Definition
    ///
    /// The position of the input-descriptor, field, and filter flags matter in this command.
    /// Nested attributes such as field and filter are associated and nested within the
    /// last input-descriptor or field seen on the CLI.
    ///
    /// The values of input descriptor are expected to be a colon-separated string in the
    /// order of "id:name:purpose". Values can be ommitted if they aren't needed.
    ///
    /// The values of field are expected to be a colon-separated string in the
    /// order of "id:name:path:purpose:optional:predicate". Values can be ommitted
    /// if they aren't needed.
    ///
    /// The values of filter are expected to be a colon-separated string in the
    /// order of "kind:pattern:value". Values can be ommitted if they aren't needed.
    ///
    /// Example:
    ///
    /// web5 pd create my-pd-3 \
    ///   --name "Complex PD" \
    ///   --purpose "Comprehensive Verification" \
    ///   --input-descriptor "input-1:Personal Info:Verify personal information" \
    ///   --field "field-1:Name:$.credentialSubject.name:Verify name:false:required" \
    ///   --filter "string:^[A-Za-z ]+$" \
    ///   --field "field-2:Age:$.credentialSubject.age:Verify age::preferred" \
    ///   --filter "number:^[0-9]+$" \
    ///   --input-descriptor "input-2:Address:Verify address" \
    ///   --field "field-3:Street:$.credentialSubject.address.street:Verify street:false:required"
    #[command(verbatim_doc_comment)]
    Create(create::CreatePresentationDefinition),
}

impl Commands {
    pub async fn command(self) {
        match self {
            Commands::Create(args) => create::run_create_command(args),
        };
    }
}
