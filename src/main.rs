use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "junbitsu", about = "Zero-touch device provisioning")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Provision a device from a YAML config
    Provision {
        /// Path to the provisioning config YAML
        #[arg(name = "config.yaml")]
        config: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Provision { config } => {
            println!("junbitsu: provisioning from {config}");
        }
    }
}
