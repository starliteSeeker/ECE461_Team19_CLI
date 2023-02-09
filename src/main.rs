mod metrics;

use clap::{Parser, Subcommand};
use log::{debug, info, LevelFilter};
use std::{io::{BufRead, BufReader}, collections::HashMap};
use metrics::Metrics;
use metrics::github::Github;
use std::io::Write;

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
        Commands::Url { url_file: f } => calcscore(f), //println!("url: {:?}", f),
        Commands::Report { report_file: f } => println!("test: {:?}", f),
    }

    fn calcscore(f: &String) {
        let mut net_scores = Vec::new();

        let file = std::fs::File::open(f).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            // if type is github
            if let Some(domain) = reqwest::Url::parse(&line).unwrap().domain() {
                println!("{}", domain);
                // calculate scores
                let project:Box<dyn Metrics> = Box::new(Github::with_url(&line).unwrap());
                let mut net_score = HashMap::new();
                let ramp_up: f64 = project.ramp_up_time();
                let correctness: f64 = project.correctness();
                let bus_factor: f64 = project.bus_factor();
                let responsiveness: f64 = project.responsiveness();
                let compatibility: f64 = project.compatibility();
                let score: f64 = ramp_up * 0.05 + correctness * 0.1 + bus_factor * 0.1 + responsiveness * 0.25 + compatibility * 0.5;
                net_score.insert("URL", line);
                net_score.insert("NET_SCORE", score.to_string());
                net_score.insert("RAMP_UP_SCORE", ramp_up.to_string());
                net_score.insert("CORRECTNESS_SCORE", correctness.to_string());
                net_score.insert("BUS_FACTOR_SCORE", bus_factor.to_string());
                net_score.insert("RESPONSIVE_MAINTAINER_SCORE", responsiveness.to_string());
                net_score.insert("LICENSE_SCORE", compatibility.to_string());
                net_scores.push(net_score);
                // sort by net scores
                net_scores.sort_by(|a, b| b["NET_SCORE"].parse::<f64>().unwrap().partial_cmp(&a["NET_SCORE"].parse::<f64>().unwrap()).unwrap());
            } else if let Some(domain) = reqwest::Url::parse(&line).unwrap().domain(){
                println!("{}", domain);
            } else {
                println!("neither");
            }
        }

        // stdout the output
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        for dict in net_scores {
            handle.write_fmt(format_args!("{{")).unwrap();
            for (key, value) in dict {
                if key != "LICENSE_SCORE" {
                    handle.write_fmt(format_args!("{}:{}, ", key, value)).unwrap();
                }
                else {
                    handle.write_fmt(format_args!("{}:{}}}\n", key, value)).unwrap();
                }   
            }
        }
    }
}
