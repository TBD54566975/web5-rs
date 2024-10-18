use std::collections::HashMap;

use crate::CheckType;

pub struct HealthCheckState {
    cli_version: Option<Option<String>>,
    dependencies: Option<HashMap<String, bool>>,
    env_vars: Option<HashMap<String, bool>>,
    connectivity: Option<bool>,
    basic_functionality: Option<bool>,
}

impl HealthCheckState {
    fn new() -> Self {
        Self {
            cli_version: None,
            dependencies: None,
            env_vars: None,
            connectivity: None,
            basic_functionality: None,
        }
    }
}

fn check_cli_version() -> Option<Option<String>> {
    Some(Some(env!("CARGO_PKG_VERSION").to_string())) // This will return the binary version.
}

fn check_dependencies() -> Option<HashMap<String, bool>> {
    // TODO : Implement this function
    // DOUBT  : Are we expecting to check system dependencies or dependencies in Cargo.toml ?
    // If cargo dependencies then how exactly shall we check the versions ?
    None
}

fn check_environment_variables() -> Option<HashMap<String, bool>> {
    let mut env_vars = HashMap::new();
    let vars = vec!["PORTABLE_DID"]; // Add env_vars that you want to include in health checkup

    for var in vars {
        let status = std::env::var(var).is_ok();
        env_vars.insert(var.to_string(), status);
    }

    Some(env_vars)
}

async fn check_connectivity() -> Option<bool> {
    let client = reqwest::Client::new();
    Some(client.get("https://developer.tbd.website/projects/web5/").send().await.is_ok())
}

fn test_basic_functionality() -> Option<bool> {
    // Supporting for actual binary
    let web5_help = std::process::Command
        ::new("web5")
        .arg("did")
        .arg("create")
        .arg("dht")
        .output()
        .is_ok();

    // Supporting for cargo environment just for testing purposes.
    let cargo_check = std::process::Command
        ::new("cargo")
        .arg("run")
        .arg("--")
        .arg("did")
        .arg("create")
        .arg("dht")
        .output()
        .is_ok();

    // If any one of the above commands is successful then return true.
    Some(web5_help || cargo_check)
}

pub async fn run_health_checks(check: Option<CheckType>) -> HealthCheckState {
    let mut state = HealthCheckState::new();

    match check {
        // Run specific checks
        Some(CheckType::CliVersion) => {
            state.cli_version = check_cli_version();
        }
        Some(CheckType::Dependencies) => {
            state.dependencies = check_dependencies();
        }
        Some(CheckType::EnvVars) => {
            state.env_vars = check_environment_variables();
        }
        Some(CheckType::Connectivity) => {
            state.connectivity = check_connectivity().await;
        }
        Some(CheckType::BasicFunctionality) => {
            state.basic_functionality = test_basic_functionality();
        }
        None => {
            // Run all checks
            state.cli_version = check_cli_version();
            state.dependencies = check_dependencies();
            state.env_vars = check_environment_variables();
            state.connectivity = check_connectivity().await;
            state.basic_functionality = test_basic_functionality();
        }
    }

    state
}

use colored::*; // Add this line at the top of your file

pub fn print_health_check_results(state: &HealthCheckState) {
    println!("{}", "Running Health Check for web5 CLI...".bold().blue());

    // Handle CLI version
    if let Some(cli_version) = &state.cli_version {
        match cli_version {
            Some(version) => println!("{} {}", "✔ CLI Version:".green(), version),
            None =>
                println!(
                    "{} {}",
                    "✖ CLI Version check failed.".red(),
                    "Please ensure the CLI is installed correctly.".yellow()
                ),
        }
    }

    // Handle dependencies
    if let Some(dependencies) = &state.dependencies {
        for (dep, status) in dependencies {
            println!(
                "{} {}: {}",
                if *status {
                    "✔".green()
                } else {
                    "✖".red()
                },
                "Dependency".bold(),
                dep
            );
            if !status {
                println!(
                    "{} {}",
                    "Remediation:".yellow(),
                    format!("Please install or update the dependency: {}", dep).yellow()
                );
            }
        }
    }

    // Handle environment variables
    if let Some(env_vars) = &state.env_vars {
        for (var, status) in env_vars {
            println!(
                "{} : {}",
                if *status {
                    "✔ Environment Variable :".green()
                } else {
                    "✖ Missing Environment Variable :".red()
                },
                if *status {
                    var.green()
                } else {
                    var.red()
                }
            );
            if !status {
                println!("{}", format!("Please set the environment variable: {}", var).yellow());
                // Example code to set the environment variable
                println!("{}", format!("export {}=your_value", var).bright_yellow());
            }
        }
    }

    // Handle connectivity
    if let Some(connectivity) = state.connectivity {
        println!(
            "{} {}",
            if connectivity {
                "✔ Connectivity:".green()
            } else {
                "✖ Connectivity:".red()
            },
            if connectivity {
                "OK".green()
            } else {
                "FAILED".red()
            }
        );
        if !connectivity {
            println!("{}", "Please check your internet connection and try again.".yellow());
        }
    }

    // Handle basic functionality
    if let Some(basic_functionality) = state.basic_functionality {
        println!(
            "{} {}",
            if basic_functionality {
                "✔ Basic CLI Functionality:".green()
            } else {
                "✖ Basic CLI Functionality:".red()
            },
            if basic_functionality {
                "OK".green()
            } else {
                "FAILED".red()
            }
        );
        if !basic_functionality {
            println!(
                "{}",
                "Might be a bug or your CLI have not been setup correctly. Please report on https://github.com/TBD54566975/web5-rs/issues ".yellow()
            );
        }
    }
}
