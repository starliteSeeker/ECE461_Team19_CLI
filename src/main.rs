mod req;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    req::stuff().unwrap();
    panic!("stop here");
    // set logging level
    let level = std::env::var("LOG_LEVEL")
        .ok()
        .and_then(|i| i.parse::<u8>().ok());
    let mut filter = match level {
        Some(2) => LevelFilter::Debug,
        Some(1) => LevelFilter::Info,
        _ => LevelFilter::Off,
    };

    // set log output
    let log_output = if filter == LevelFilter::Off {
        env_logger::fmt::Target::Stderr // can be anything
    } else {
        let fp = std::env::var("LOG_FILE")
            .ok()
            .and_then(|i| std::fs::File::create(i).ok());
        if fp.is_none() {
            // turn off loggin gif log file not found
            filter = LevelFilter::Off;
            env_logger::fmt::Target::Stderr // can be anything
        } else {
            env_logger::fmt::Target::Pipe(Box::new(fp.unwrap()))
        }
    };

    // setup logger
    env_logger::Builder::new()
        .filter_level(filter)
        .target(log_output)
        .init();

    info!("print info");
    debug!("print debug");

    // parse command line arguments
    let cli = Cli::parse();

    match &cli.command {
        Commands::Url { url_file: f } => println!("url: {:?}", f),
        Commands::Report { report_file: f } => println!("test: {:?}", f),
    }

    Ok(())
}
