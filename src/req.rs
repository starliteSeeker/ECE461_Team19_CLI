use reqwest::Result;

pub fn stuff() -> Result<()> {
    let s = "{\"query\": \"query { repository(owner:\\\"ppy\\\", name:\\\"osu\\\") { mentionableUsers { totalCount } } }\"}";
    println!("{}", s);
    let client = reqwest::blocking::Client::builder()
        .user_agent("ECE461_Team19_CLI")
        .build()?;
    let body = client
        .post("https://api.github.com/graphql")
        .bearer_auth(env!("GITHUB_TOKEN"))
        .body(s)
        .send()?
        .text()?;

    println!("{}", body);

    Ok(())
}
