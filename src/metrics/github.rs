use crate::metrics::Metrics;
use reqwest::header;
use statrs::distribution::{Continuous, Normal};
use std::io::BufRead;
use async_trait::async_trait;

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

        // check if scheme is https or http
        let sch = u.scheme();
        if sch != "https" && sch != "http" {
            return None;
        }

        // extract repo info from url
        let mut path = u.path().split('/').skip(1);
        let link = url.to_string();
        let owner = path.next()?.to_string();
        let repo = path.next()?.to_string();

        // http client
        let mut headers = header::HeaderMap::new();
        let t = format!("Bearer {}", std::env::var("GITHUB_TOKEN").ok()?);
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&t).ok()?,
        );
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
            .send()
    }

    // REST API call with result in json format
    pub fn rest_json(&self, path: &str) -> reqwest::Result<serde_json::Value> {
        self.rest_api(path)?.json::<serde_json::Value>()
    }

    // GitHub GraphQL API 
    pub async fn graphql_api(&self) -> reqwest::Result<serde_json::Value> {
        let client = reqwest::Client::builder();
        let response = client
            .user_agent("ECE461_Team19_CLI")
            .build()
            .unwrap()
            .post("https://api.github.com/graphql")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .bearer_auth(format!("{}", std::env::var("GITHUB_TOKEN").unwrap()))
            .body(
                format!("{{\"query\" : \"query {{ repository(owner:\\\"{}\\\", name:\\\"{}\\\") {{ mentionableUsers {{ totalCount }} }} }}\" }}", self.owner, self.repo)
            )
            .send()
            .await?;
        
        return Ok(response.json::<serde_json::Value>().await?);
    }

    // count how many pages the result has
    // see: https://docs.github.com/en/rest/guides/using-pagination-in-the-rest-api?apiVersion=2022-11-28
    pub fn rest_page_count(&self, path: &str) -> reqwest::Result<u32> {
        let response = self.rest_api(path)?;
        let header = response.headers().get("link");
        if header.is_none() {
            if response.json::<serde_json::Value>().unwrap()["message"].is_null() {
                return Ok(1);
            } else {
                return Ok(0);
            }
        }

        // get substring with the page number
        let res = header.unwrap().to_str().unwrap().split(',').nth(1).unwrap();
        // get page number
        let page = res.get(res.find("&page=").unwrap() + 6..res.find('>').unwrap());

        Ok(page.unwrap().parse::<u32>().unwrap())
    }
}
#[async_trait] 
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
                return 0.0;
            }
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
        // issues returns pull requests as well, so subtract pulls from issues
        let all = self.rest_page_count("issues?state=all&per_page=1").unwrap()
            - self.rest_page_count("pulls?state=all&per_page=1").unwrap();
        let closed = self
            .rest_page_count("issues?state=closed&per_page=1")
            .unwrap()
            - self
                .rest_page_count("pulls?state=closed&per_page=1")
                .unwrap();
        if all == 0 {
            0.0
        } else {
            closed as f64 / all as f64
        }
    }

    async fn bus_factor(&self) -> f64 {
        let bus = self.graphql_api().await.unwrap(); 
        let stringData = bus["data"]["repository"]["mentionableUsers"]["totalCount"].as_str();
        let mut x: f64 = stringData.unwrap().parse().unwrap();
        x = ((2.0 * x) / (x + 1.0)) - 1.0 

    }

    fn responsiveness(&self) -> f64 {
        0.0
    }

    fn compatibility(&self) -> f64 {
        // get license with github api
        let l = self.rest_json("license").unwrap();
        let license = l["license"]["spdx_id"].as_str();

        // no license found
        if license.is_none() {
            return 0.0;
        }

        let acceptable = [
            "LGPL-2.1-only",
            "LGPL-2.1",
            "LGPL-2.1-or-later",
            "LGPL-3.0-only",
            "LGPL-3.0",
            "BSD-3-Clause",
            "MIT",
            "X11",
        ];
        if acceptable.contains(&license.unwrap()) {
            1.0
        } else {
            0.0
        }
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
