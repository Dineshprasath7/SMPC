use clap::{Parser, Subcommand};
use smpc::shamir;
use std::io;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let home =
        dirs::home_dir().ok_or_else(|| io::Error::other("could not determine home directory"))?;
    let smpc = shamir::Smpc::new(&home)?;

    match cli.command {
        Commands::Split => {
            if let Err(e) = command_line::split_cli(&smpc) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }

        Commands::Reconstruct => {
            if let Err(e) = command_line::reconstruct_cli(&smpc) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }
    Ok(())
}
