mod metrics;

use clap::{Parser, Subcommand};
use log::{debug, info, LevelFilter};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Variables {
    owner: String,
    repo: String,
}

#[derive(Serialize, Debug)]
struct RequestBody<'a> {
    query: &'a str,
    variables: Variables,
}

#[derive(Deserialize, Debug)]
struct CollaboratorConnection {
    total_count: i32,
}

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    collaborators: CollaboratorConnection,
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    repository: Repository,
}

#[derive(Deserialize, Debug)]
struct ResponseBody {
    data: ResponseData,
}

async fn graphql_api(client: &Client, token: &str, link: &str) -> Option<i32> {
    let query = r#"
        query ($owner: String!, $repo: String!) {
            repository(owner: $owner, name: $repo) { 
                name
                collaborators(first: 0) {
                    totalCount 
                }
            }
        }
    "#;

    let link_split: Vec<&str> = link.split('/').collect();
    let owner = link_split[3];
    let repo = link_split[4];

    let variables = Variables {
        owner: owner.to_owned(),
        repo: repo.to_owned(),
    };
    let request_body = RequestBody { query, variables };

    let response = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .json(&request_body)
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                let response_body: ResponseBody = response.json().await.unwrap();
                Some(response_body.data.repository.collaborators.total_count)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

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

#[tokio::main]
async fn main() {
    // Testing GraphQL
    let client = Client::new();
    let token = "token";
    let link = "https://github.com/rust-lang/rust";
    // match graphql_api(&client, token, link).await {
    //     Some(collaborators_count) => println!("Number of collaborators: {}", collaborators_count),
    //     None => println!("Failed to get collaborator count"),
    // }

    let collaborators = graphql_api(&client, token, link).await;
    println!("num of collaborators: {:?}", collaborators);

    println!("Hello");
    panic!("Err");

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
}
