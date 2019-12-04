extern crate reqwest;

pub use crate::credential::{BaseAuthCredential};

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{Value};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

pub struct JiraIssue {
    pub title: String,
    pub url: String
}

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
const GIRHUBENDPOINR: &str = "https://api.github.com/repos/wear";


pub fn fetch_issue(issue: &str, credential: &BaseAuthCredential) -> Result<JiraIssue, Box<dyn std::error::Error>> {
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

        let jira_issue = JiraIssue {
            title: v["fields"]["summary"].to_string().replace(" ", "-"),
            url: reqeust_url
        };
        Ok(jira_issue)
    } else {
        let request_error = RequestError::from(status_code.to_string());
        Err(Box::new(request_error))
    }
}


pub fn create_issue(issue: &JiraIssue, credential: &BaseAuthCredential) -> Result<String, Box<dyn std::error::Error>> {
    let secret = credential.secret();

    let reqeust_url = format!("{}/jiraify/issues",GIRHUBENDPOINR);
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("title", issue.title.clone());
    let body = format!("refs: {}", issue.url);
    map.insert("body", body);

    let mut resp = client.post(&reqeust_url)
      .header(AUTHORIZATION, &secret)
      .header(CONTENT_TYPE, "application/json")
      .json(&map)
      .send()?;

    let status_code = resp.status();
    let v: Value = serde_json::from_str(&resp.text()?)?;

    if status_code.is_success() {
        Ok(v["url"].to_string())
    } else {
        let request_error = RequestError::from(v["message"].to_string());
        Err(Box::new(request_error))
    }
}