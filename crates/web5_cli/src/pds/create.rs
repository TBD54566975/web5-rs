use clap::{arg, Args, FromArgMatches};
use web5::credentials::presentation_definition::*;

#[derive(Debug)]
pub struct CreatePresentationDefinition(PresentationDefinition);

impl CreatePresentationDefinition {
    pub fn get_output(self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }
}

impl Args for CreatePresentationDefinition {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        cmd.arg(arg!(<id> "A unique identifier that distinguishes this PD from others"))
            .arg(arg!(-n --name [name] "A human-friendly name for easier identification of the PD").required(false))
            .arg(arg!(-p --purpose [purpose] "A description outlining why the information requested by the PD is needed").required(false))
            .arg(arg!(-i --"input-descriptor" [input_descriptor] ... "Required claims and specifications on exactly how they will be evaluated").required(false))
            .next_help_heading("Input Descriptor Info")
            .arg(arg!(-d --field [field] ... "Represents a specific piece of data that the PD is interested in. Each field can have its own constraints.").required(false))
            .next_help_heading("Field Constraint Info")
            .arg(arg!(-f --filter [filter] ... "The specific conditions that the data in the specified path must satisfy").required(false))
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        Self::augment_args(cmd)
    }
}

impl FromArgMatches for CreatePresentationDefinition {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let id = matches.get_one::<String>("id").ok_or(clap::Error::new(
            clap::error::ErrorKind::MissingRequiredArgument,
        ))?;
        let name = matches.get_one::<String>("name");
        let purpose = matches.get_one::<String>("purpose");

        let ids_idx = matches
            .indices_of("input-descriptor")
            .unwrap_or_default()
            .collect::<Vec<_>>();
        let ids = matches
            .get_many::<String>("input-descriptor")
            .unwrap_or_default()
            .collect::<Vec<_>>();
        let ids_count = ids.len();
        let fds_idx = matches
            .indices_of("field")
            .unwrap_or_default()
            .collect::<Vec<_>>();
        let fds = matches
            .get_many::<String>("field")
            .unwrap_or_default()
            .collect::<Vec<_>>();
        let fds_count = fds.len();
        let fts_idx = matches
            .indices_of("filter")
            .unwrap_or_default()
            .collect::<Vec<_>>();
        let fts = matches
            .get_many::<String>("filter")
            .unwrap_or_default()
            .collect::<Vec<_>>();
        let fts_count = fts.len();

        // for each input descriptor location on CLI
        let mut current_fd = 0;
        let mut current_ft = 0;
        let mut input_descriptors = Vec::new();
        for (i, _) in ids_idx.iter().enumerate() {
            let mut input_descriptor = parse_input_descriptor(ids[i]);
            // for each field within current and next input descriptor index
            while i + 1 < ids_count && fds_idx[current_fd] < ids_idx[i + 1] {
                let mut field = parse_field(fds[current_fd]);
                // for each filter within current and next field index
                while current_fd + 1 < fds_count
                    && current_ft + 1 < fts_count
                    && fts_idx[current_ft] < fds_idx[current_fd + 1]
                    && fts_idx[current_ft] < ids_idx[i + 1]
                {
                    let filter = parse_filter(fts[current_ft]);
                    field.filter = Some(filter);
                    current_ft += 1;
                }

                input_descriptor.constraints.fields.push(field);
                current_fd += 1;
            }

            input_descriptors.push(input_descriptor);
        }

        let definition = PresentationDefinition {
            id: id.clone(),
            name: name.cloned(),
            purpose: purpose.cloned(),
            input_descriptors,
            submission_requirements: None,
        };

        Ok(CreatePresentationDefinition(definition))
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        self.0.id = matches
            .get_one::<String>("id")
            .ok_or(clap::Error::new(
                clap::error::ErrorKind::MissingRequiredArgument,
            ))?
            .to_string();
        self.0.name = matches.get_one::<String>("name").cloned();
        self.0.purpose = matches.get_one::<String>("purpose").cloned();

        let ids_idx = matches
            .indices_of("input-descriptor")
            .unwrap_or_default()
            .collect::<Vec<_>>();
        let ids = matches
            .get_many::<String>("input-descriptor")
            .unwrap_or_default()
            .collect::<Vec<_>>();
        let ids_count = ids.len();
        let fds_idx = matches
            .indices_of("field")
            .unwrap_or_default()
            .collect::<Vec<_>>();
        let fds = matches
            .get_many::<String>("field")
            .unwrap_or_default()
            .collect::<Vec<_>>();
        let fds_count = fds.len();
        let fts_idx = matches
            .indices_of("filter")
            .unwrap_or_default()
            .collect::<Vec<_>>();
        let fts = matches
            .get_many::<String>("filter")
            .unwrap_or_default()
            .collect::<Vec<_>>();
        let fts_count = fts.len();

        // for each input descriptor location on CLI
        let mut current_fd = 0;
        let mut current_ft = 0;
        for (i, _) in ids_idx.iter().enumerate() {
            let mut input_descriptor = parse_input_descriptor(ids[i]);
            // for each field within current and next input descriptor index
            while i + 1 < ids_count && fds_idx[current_fd] < ids_idx[i + 1] {
                let mut field = parse_field(fds[current_fd]);
                // for each filter within current and next field index
                while current_fd + 1 < fds_count
                    && current_ft + 1 < fts_count
                    && fts_idx[current_ft] < fds_idx[current_fd + 1]
                    && fts_idx[current_ft] < ids_idx[i + 1]
                {
                    let filter = parse_filter(fts[current_ft]);
                    field.filter = Some(filter);
                    current_ft += 1;
                }

                input_descriptor.constraints.fields.push(field);
                current_fd += 1;
            }

            self.0.input_descriptors.push(input_descriptor);
        }

        Ok(())
    }
}

fn parse_input_descriptor(value: &str) -> InputDescriptor {
    let mut splits = value.split(':');
    let id = splits.next().unwrap();
    let name = splits.next().unwrap_or_default();
    let purpose = splits.next().unwrap_or_default();

    InputDescriptor {
        id: id.to_string(),
        name: str_to_option_string(name),
        purpose: str_to_option_string(purpose),
        constraints: Constraints { fields: Vec::new() },
    }
}

fn parse_field(value: &str) -> Field {
    let mut splits = value.split(':');
    let id = splits.next().unwrap_or_default();
    let name = splits.next().unwrap_or_default();
    let path = splits.next().unwrap_or_default();
    let purpose = splits.next().unwrap_or_default();
    let optional = splits.next().unwrap_or_default();
    let predicate = splits.next().unwrap_or_default();

    Field {
        id: str_to_option_string(id),
        name: str_to_option_string(name),
        path: path.split(',').map(String::from).collect(),
        purpose: str_to_option_string(purpose),
        filter: None,
        optional: (!optional.is_empty()).then(|| optional == "true"),
        predicate: str_to_optionality(predicate),
    }
}

fn parse_filter(value: &str) -> Filter {
    let mut splits = value.split(':');
    let kind = splits.next().unwrap_or_default();
    let pattern = splits.next().unwrap_or_default();
    let value = splits.next().unwrap_or_default();

    Filter {
        r#type: str_to_option_string(kind),
        pattern: str_to_option_string(pattern),
        const_value: str_to_option_string(value),
        contains: None,
    }
}

fn str_to_optionality(value: &str) -> Option<Optionality> {
    match value {
        "required" => Some(Optionality::Required),
        "preferred" => Some(Optionality::Preferred),
        _ => None,
    }
}

fn str_to_option_string(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_string())
}

pub fn run_create_command(args: CreatePresentationDefinition) {
    println!("{}", args.get_output());
}
