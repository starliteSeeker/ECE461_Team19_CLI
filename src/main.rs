use clap::{Parser, Subcommand};

// command line argumand parser
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    // Print modules in order of trustworthiness
    Url { url_file: String },

    // Parse results of tests
    Report { report_file: String },
}

fn main() {
    // parse command line arguments
    let cli = Cli::parse();

    match &cli.command {
        Commands::Url { url_file: f } => println!("url: {:?}", f),
        Commands::Report { report_file: f } => println!("test: {:?}", f),
    }
}
