use clap::{Parser, Subcommand};

mod command_line;

#[derive(Parser, Debug)]
#[command(
    name = "smpc",
    version,
    about = "Secure Multi-Party Computation utilities"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Split a secret into Shamir shares
    Split,

    /// Reconstruct a secret from Shamir shares
    Reconstruct,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Split => {
            if let Err(e) = command_line::split_cli() {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }

        Commands::Reconstruct => {
            if let Err(e) = command_line::reconstruct_cli() {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }
}
