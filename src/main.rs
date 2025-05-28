mod commands;
mod files;

use clap::Parser;
use commands::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::BuildImage {} => {
            println!(
                "Building image with Airflow version: {:?}",
                cli.airflow_version
            );
            // TODO
        }
        Commands::Start { project_name } => {
            println!("Starting local Airflow with project name: {}", project_name);
            // TODO
        }
        Commands::ResetDb { project_name } => {
            println!(
                "Resetting local PostgresDB container with project name: {}",
                project_name
            );
            // TODO
        }
        Commands::TestRequirments { skip_build } => {
            println!(
                "Testing requirements with Airflow version: {} and skip_build: {}",
                cli.airflow_version, skip_build
            );
            // TODO
        }
        Commands::PackageRequirements { whl_path } => {
            println!("Packaging requirements with WHL path: {}", whl_path);
            // TODO
        }
    }
}
