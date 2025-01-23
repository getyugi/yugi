use clap::Parser;

mod cli;
mod commands;
#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Build { dir: _ } => todo!(),
        cli::Commands::New { name } => commands::new::new_template(name.as_str()).await,
        cli::Commands::Serve { port: _ } => todo!(),
    }
}
