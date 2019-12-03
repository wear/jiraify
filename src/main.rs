mod credential;
mod issue_controller;

pub use crate::credential::{JiraCredential, GitbhubCredential};
pub use crate::issue_controller::JiraIssue;

extern crate clap;
use clap::{Arg, App};
use std::io;

fn main()  {
    const JIRACONFIG: &str = "jira2-conf";
    const GITHUBCONFIG: &str = "github2-conf";

    let mut jira_config: JiraCredential = confy::load(JIRACONFIG).unwrap();
    let mut github_config: GitbhubCredential = confy::load(GITHUBCONFIG).unwrap();

    let matches = App::new("Jiraify")
       .version("1.0")
       .about("Automate move Jira issue to github!")
       .author("Stephen Kong.")
       .arg(
            Arg::with_name("issue")
           .short("i")
           .long("issue")
           .takes_value(true)
           .help("Move issue")
        )
       .arg(
            Arg::with_name("config")
           .short("c")
           .long("config")
           .help("Update github config")
        )
       .get_matches();

    if matches.is_present("issue") {
        let issue_id = matches.value_of("issue").unwrap();

        match issue_controller::fetch_issue(issue_id, &jira_config) {
            Ok(issue) => {
              let resp = issue_controller::create_issue(&issue, &github_config);
              println!("{:?}", &resp);
            }
            Err(error) => { println!("Problem fetch JIRA issue: {:?}", error) },
        }
    }

    if matches.is_present("config") {
        let mut input = String::new();

        println!("Please input your JIRA email.");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        jira_config.email = input.trim().to_string();
        input.clear();

        println!("Please input your JIRA api token.");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        jira_config.api_token = input.trim().to_string();
        input.clear();

        confy::store(JIRACONFIG, &jira_config).unwrap();

        println!("Please input your Github user name.");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        github_config.user_name = input.trim().to_string();
        input.clear();

        println!("Please input your Github access token.");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        github_config.access_token = input.trim().to_string();
        input.clear();

        confy::store(GITHUBCONFIG, &github_config).unwrap();
    }
}



