use reqwest::Result;

pub fn stuff() -> Result<()> {
    let github_token = std::env::var("GITHUB_TOKEN").unwrap();
    let client = reqwest::blocking::Client::builder()
        .user_agent("lee3445")
        .build()?;
    let body = client
        .get("https://api.github.com/repos/seanmonstar/reqwest/stargazers")
        // .get("https://api.github.com/octocat")
        .header(reqwest::header::AUTHORIZATION, github_token)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()?
        .text()?;

    println!("{}", body);

    Ok(())
}
