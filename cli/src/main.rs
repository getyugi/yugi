use clap::Parser;
use std::env;
use yugi::utils::path::str_to_absolute_path;

mod cli;
mod commands;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    let binding = env::current_dir().unwrap();
    let wd = binding.to_str().unwrap();
    match cli.command {
        cli::Commands::Build { dir } => {
            commands::build::build_project(
                str_to_absolute_path(&wd).as_path(),
                str_to_absolute_path(&dir).as_path(),
            )
            .await
        }
        cli::Commands::New { name } => commands::new::new_template(name.as_str()).await,
        cli::Commands::Serve { port: _ } => todo!(),
    }
}
