use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "yugi")]
#[command(version ,about = "A Static Site Generator", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Build the site
    Build {
        #[arg(short, long, default_value_t = String::from("output"))]
        dir: String,
    },
    /// Create a new site project
    New {
        /// Name of the project
        name: String,
    },
    /// Serve the site locally
    Serve {
        /// Port number
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{Cli, Commands};

    #[test]
    fn test_new_command_parsing() {
        let args = vec!["yugi", "new", "my_project"];

        let cli = Cli::try_parse_from(args).unwrap();
        // Match the parsed command

        match cli.command {
            Commands::New { name } => assert_eq!(name, "my_project"),
            _ => panic!("Expected 'New' command"),
        }
    }

    #[test]
    fn test_serve_command_with_default_port_parsing() {
        let args = vec!["yugi", "serve"];

        let cli = Cli::try_parse_from(args).unwrap();
        // Match the parsed command

        match cli.command {
            Commands::Serve { port } => assert_eq!(port, 8080),
            _ => panic!("Expected 'Serve' command"),
        }
    }

    #[test]
    fn test_serve_command_with_custom_port_parsing() {
        let args = vec!["yugi", "serve", "--port", "3000"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Serve { port } => assert_eq!(port, 3000),
            _ => panic!("Expected 'Serve' command"),
        }
    }

    #[test]
    fn test_build_command_with_default_dir_parsing() {
        let args = vec!["yugi", "build"];

        let cli = Cli::try_parse_from(args).unwrap();
        // Match the parsed command

        match cli.command {
            Commands::Build { dir } => assert_eq!(dir, "output"),
            _ => panic!("Expected 'Build' command"),
        }
    }

    #[test]
    fn test_build_command_with_custom_dir_parsing() {
        let args = vec!["yugi", "build", "--dir", "result"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Build { dir } => assert_eq!(dir, "result"),
            _ => panic!("Expected 'Build' command"),
        }
    }
}
