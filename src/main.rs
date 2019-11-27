extern crate reqwest;
extern crate base64;
#[allow(dead_code)]

use structopt::StructOpt;
use base64::{encode};
use std::collections::HashMap;
use reqwest::ClientBuilder;
use reqwest::header;

#[derive(StructOpt)]
struct Cli {
    issue: String
}



fn main() -> Result<(), Box<dyn std::error::Error>>  {
    // let secret = format!("{}:{}", "stephen.kong@managebac.com", "pu00YEfVflOWLKY3BDtBD1EC");

    // let mut headers = header::HeaderMap::new();
    // headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&secret).unwrap());

    let args = Cli::from_args();
    let request_url = format!("https://fariaedu.atlassian.net/rest/api/2/issue/{}", &args.issue);

    let client = ClientBuilder::new().build()?;
    let response = client.basic_auth("stephen.kong@managebac.com", "pu00YEfVflOWLKY3BDtBD1EC").head(&request_url).send()?;

    if response.status().is_success() {
        println!("{} is a user!", "dsfadsf");
    } else {
        println!("{:?} is not a user!", response);
    }


    Ok(())
}
