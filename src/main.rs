use clap::{Parser, Subcommand};
use log::{debug, info, LevelFilter};

// command line argumand parser
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Print modules in order of trustworthiness
    Url { url_file: String },

    /// Parse results of tests
    Report { report_file: String },
}

fn main() {
    // setup logging level
    let level = std::env::var("LOG_LEVEL")
        .ok()
        .and_then(|i| i.parse::<u8>().ok());
    let filter = match &level {
        Some(2) => LevelFilter::Debug,
        Some(1) => LevelFilter::Info,
        _ => LevelFilter::Off,
    };
    env_logger::Builder::new().filter_level(filter).init();

    info!("print info");
    debug!("print debug");

    // parse command line arguments
    let cli = Cli::parse();

    match &cli.command {
        Commands::Url { url_file: f } => println!("url: {:?}", f),
        Commands::Report { report_file: f } => println!("test: {:?}", f),
    }
}
