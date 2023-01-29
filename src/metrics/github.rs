use crate::metrics::Metrics;

#[derive(Debug)]
pub struct Github {
    owner: String,
    repo: String,
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
        let owner = path.next()?;
        let repo = path.next()?;

        Some(Github {
            owner: owner.to_string(),
            repo: repo.to_string(),
        })
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
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_with_url() {
        let a = Github::with_url("https://github.com/lee3445/ECE461_Team19_CLI").unwrap();

        assert_eq!(a.owner, "lee3445");
        assert_eq!(a.repo, "ECE461_Team19_CLI");
    }

    #[test]
    fn construct_with_bad_url() {
        assert!(Github::with_url("not an url").is_none());
    }
    #[test]
    fn construct_with_wrong_domain() {
        assert!(Github::with_url(
            "https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html"
        )
        .is_none());
    }
    #[test]
    fn construct_with_bad_github_url() {
        assert!(Github::with_url("https://github.com/rust-lang").is_none());
    }
}
