extern crate reqwest;
extern crate base64;

use structopt::StructOpt;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use base64::{encode};
use serde_json::{Value};

#[derive(StructOpt)]
struct Cli {
    issue: String, 
    email: String,
    api_token: String
}

struct Credential<'a> {
  email: &'a str,
  api_token: &'a str
}

impl<'a> Credential<'a> {
  fn secret(&self) -> String {
    let secret = &format!("{email}:{api_key}", email=self.email, api_key=self.api_token);
    encode(secret)
  }
}

fn main()  {
    let args = Cli::from_args();
    let credit = Credential {
      email: &args.email,
      api_token: &args.api_token
    };
    let secret = credit.secret();
    match fetch_issue(&args.issue, &secret) {
      Ok(content) => { println!("{}", content);},
      Err(error) => { println!("{:?}", error); }
    }
}

fn fetch_issue(issue: &str, secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    let reqeust_url = format!(
        "https://fariaedu.atlassian.net/rest/api/latest/issue/{}", issue);
    let client = reqwest::Client::new();
    let auth = format!("Basic {}", secret);

    let resp = client.get(&reqeust_url)
        .header(AUTHORIZATION, auth)
        .header(CONTENT_TYPE, "application/json")
        .send()?
        .text()?;

    let v: Value = serde_json::from_str(&resp)?;    

    Ok(v["fields"]["summary"].to_string())
}
