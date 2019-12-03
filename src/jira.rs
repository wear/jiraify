extern crate reqwest;

pub use crate::credential::JiraCredential;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{Value};
use std::error::Error;
use std::fmt;

struct RequestError {
    status_code: String
}

impl RequestError {
    fn from(status_code: String) -> Self {
       Self {
         status_code
       }
    }
}

impl Error for RequestError {
    fn description(&self) -> &str {
        "I'm SuperError side kick"
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",  self.status_code)
    }
}

impl fmt::Debug for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",  self.status_code)
    }
}

const JIRAENDPOINR: &str = "https://fariaedu.atlassian.net/rest/api/latest";


pub fn fetch_issue(issue: &str, credential: &JiraCredential) -> Result<String, Box<dyn std::error::Error>> {
    let secret = credential.secret();

    let reqeust_url = format!(
        "{endpoint}/issue/{id}",endpoint=JIRAENDPOINR, id=issue);
    let client = reqwest::Client::new();

    let mut resp = client.get(&reqeust_url)
        .header(AUTHORIZATION, &secret)
        .header(CONTENT_TYPE, "application/json")
        .send()?;

    let status_code = resp.status();

    if status_code.is_success() {
        let v: Value = serde_json::from_str(&resp.text()?)?;
        Ok(v["fields"]["summary"].to_string())
    } else {
        let request_error = RequestError::from(status_code.to_string());
        Err(Box::new(request_error))
    }


}