extern crate reqwest;

use structopt::StructOpt;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;

#[derive(StructOpt)]
struct Cli {
    issue: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let reqeust_url = format!(
        "https://fariaedu.atlassian.net/rest/api/latest/issue/{issue}", 
        issue=&args.issue);
    let client = reqwest::Client::new();
    let resp = client.get(&reqeust_url)
        .header(AUTHORIZATION, "Basic c3RlcGhlbi5rb25nQG1hbmFnZWJhYy5jb206NThvWkV2V1ZMekRtcjNETVhvQUk1QzVF")
        .header(CONTENT_TYPE, "application/json")
        .send()?
        .text()?;

    println!("{}", resp);

    Ok(())
}
