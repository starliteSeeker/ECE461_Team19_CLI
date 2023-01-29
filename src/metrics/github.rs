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
    fn correctness(&self) -> f64 {
        1.0
    }
}
