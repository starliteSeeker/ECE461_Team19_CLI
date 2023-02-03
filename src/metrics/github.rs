use crate::metrics::Metrics;
use reqwest::header;

#[derive(Debug)]
pub struct Github {
    // repository information
    owner: String,
    repo: String,

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
}
impl Metrics for Github {
    fn ramp_up_time(&self) -> f64 {
        0.0
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
        // get license with github api
        let l = self.rest_json("license").unwrap();
        let license = l["license"]["spdx_id"].as_str();

        // no license found
        if license.is_none() {
            return 0.0;
        }

        let acceptable = [
            "LGPL-2.1-only",
            "LGPL-2.1-or-later",
            "LGPL-3.0-only",
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
