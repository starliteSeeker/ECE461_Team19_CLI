use crate::metrics::Metrics;
use crate::metrics::github::Github;
use serde_json::Value;      // for parsing json


pub struct Npm {
    gh: Box<dyn Metrics>,
    derefurl: String,
}

impl Npm {
    pub fn with_url(url: &str) -> Option<Npm> {
        let resp = reqwest::blocking::get(url).unwrap().text().unwrap();

        // input url
        let input: &str = &resp;

        // parse url into generic JSON value
        let root: Value = serde_json::from_str(input).unwrap();

        // access element using .get()
        let giturl: Option<&str> = root.get("repository")
            .and_then(|value| value.get("url"))
            .and_then(|value| value.as_str());
        // println!("giturl = {:?}", giturl);

        // dereference the url so we can use .replace() later
        let derefurl = &giturl.as_deref().unwrap();

        // Do not need to check if url contains git+, just do replace. That would take care of it
        // if derefurl.contains("git+") {
        //     let derefurl = derefurl.replace("git+", "");
        // }
        
        let derefurl = derefurl.replace("git+", "");
        let derefurl = derefurl.replace(".git", "");
        println!("giturl = {:?}", derefurl);
        
        // create github object
        let output = Github::with_url(&derefurl)?;
        
        // return
        Some(Npm {
            gh: Box::new(output),
            derefurl: derefurl.to_string()
        })
    }
}

impl Metrics for Npm {
    fn ramp_up_time(&self) -> f64 {
        self.gh.ramp_up_time()
    }

    fn correctness(&self) -> f64 {
        self.gh.correctness()
    }

    fn bus_factor(&self) -> f64 {
        self.gh.bus_factor()
    }

    fn responsiveness(&self) -> f64 {
        self.gh.responsiveness()
    }

    fn compatibility(&self) -> f64 {
        self.gh.compatibility()
    }
}