use crate::metrics::Metrics;
use reqwest::header;
use std::io::BufRead;
use statrs::distribution::{Continuous, Normal};

#[derive(Debug)]
pub struct Github {
    // repository information
    owner: String,
    repo: String,
    link: String,

    // API-related
    client: reqwest::blocking::Client,
}

impl Github {
    // create new instance with url
    pub fn with_url(url: &str) -> Option<Github> {
        let u = reqwest::Url::parse(url).ok()?;

        // check if domain is "github.com"
        if let Some(domain) = u.domain() {
            if domain != "github.com" {
                return None;
            }
        } else {
            return None;
        }

        // extract repo info from url
        let mut path = u.path().split('/').skip(1);
        let link = url.to_string();
        let owner = path.next()?.to_string();
        let repo = path.next()?.to_string();

        // http client
        let mut headers = header::HeaderMap::new();
        let t = std::env::var("GITHUB_TOKEN").ok()?;
        let mut token = header::HeaderValue::from_str(&t).ok()?;
        token.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, token);
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(
            "X-GitHub-Api-Version",
            header::HeaderValue::from_static("2022-11-28"),
        );
        let client = reqwest::blocking::Client::builder()
            .user_agent("ECE461_Team19_CLI")
            .default_headers(headers)
            .build()
            .ok()?;

        Some(Github {
            owner,
            repo,
            link,
            client,
        })
    }

    // GitHub REST API
    // https://docs.github.com/en/rest?apiVersion=2022-11-28
    pub fn rest_api(&self, path: &str) -> reqwest::Result<reqwest::blocking::Response> {
        self.client
            .get(format!(
                "https://api.github.com/repos/{}/{}/{}",
                self.owner, self.repo, path
            ))
            .header(
                header::ACCEPT,
                header::HeaderValue::from_static("application/vnd.github+json"),
            )
            .send()
    }

    // REST API call with result in json format
    pub fn rest_json(&self, path: &str) -> reqwest::Result<serde_json::Value> {
        self.rest_api(path)?.json::<serde_json::Value>()
    }
}
impl Metrics for Github {
    fn ramp_up_time(&self) -> f64 {
        // Specify the path of repo to clone into
        let repo_path = std::path::Path::new("cloned_repo");

        // Clone the repo
        git2::Repository::clone(&self.link, repo_path).unwrap();

        // Check if there is readme
        let file = match std::fs::File::open("cloned_repo/README.md") {
            Ok(file) => file,
            Err(_) => {
                println!("Cannot find README");
                return 0.0
            },
        };
        let reader = std::io::BufReader::new(file);

        // Get the # of lines and calculate the score
        let lines = reader.lines().count();
        let mut x = lines as f64;
        x = x / 150.0 * 0.7;
        let normal = Normal::new(0.0, 1.0).unwrap();
        let result = normal.pdf(x) * x.sqrt() / 0.261;
        std::fs::remove_dir_all(repo_path).unwrap();
        result
    }

    fn correctness(&self) -> f64 {
        0.0
    }

    fn bus_factor(&self) -> f64 {
        0.0
    }

    fn responsiveness(&self) -> f64 {
        0.0
    }

    fn compatibility(&self) -> f64 {
        0.0
    }
}

#[cfg(test)] // needs $GITHUB_TOKEN
mod tests {
    use super::*;

    // testing with_url()
    #[test]
    fn construct_with_url() {
        let a = Github::with_url("https://github.com/lee3445/ECE461_Team19_CLI").unwrap();

        assert_eq!(a.owner, "lee3445");
        assert_eq!(a.repo, "ECE461_Team19_CLI");
    }

    #[test]
    fn construct_with_bad_url() {
        // not an url
        assert!(Github::with_url("not an url").is_none());

        // not a github url
        assert!(Github::with_url(
            "https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html"
        )
        .is_none());

        // not a repo url
        assert!(Github::with_url("https://github.com/rust-lang").is_none());
    }

    // testing rest_json()
    #[test]
    fn rest_api_stargazers() -> reqwest::Result<()> {
        let g = Github::with_url("https://github.com/seanmonstar/reqwest").unwrap();
        assert_eq!(
            30,
            g.rest_json("stargazers").unwrap().as_array().unwrap().len()
        );
        Ok(())
    }
}
