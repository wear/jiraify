mod credential;
mod jira;

pub use crate::credential::JiraCredential;

extern crate clap;
use clap::{Arg, App};
use std::io;

fn main()  {
    const JIRACONFIG: &str = "jira-conf";

    let mut jira_config: JiraCredential = confy::load(JIRACONFIG).unwrap();

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
           .help("Update config")
        )
       .get_matches();


    if matches.is_present("issue") {
        let issue = matches.value_of("issue").unwrap();
        match jira::fetch_issue(issue, &jira_config) {
            Ok(summary) => { println!("issue summary is {}", summary); }
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
    }
}



