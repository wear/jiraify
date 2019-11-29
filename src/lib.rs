extern crate reqwest;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{Value};

pub fn fetch_issue(issue: &str, secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    let reqeust_url = format!(
        "https://fariaedu.atlassian.net/rest/api/latest/issue/{}", issue);
    let client = reqwest::Client::new();
    let resp = client.get(&reqeust_url)
        .header(AUTHORIZATION, secret)
        .header(CONTENT_TYPE, "application/json")
        .send()?
        .text()?;

    let v: Value = serde_json::from_str(&resp)?;

    Ok(v["fields"]["summary"].to_string())
}