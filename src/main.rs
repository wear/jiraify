mod credential;
mod jira;
pub use crate::credential::JiraCredential;


extern crate clap;
use clap::{Arg, App, SubCommand};


fn main()  {
    let mut jira_config: JiraCredential = confy::load("jira-config").unwrap();

    let matches = App::new("Jiraify")
       .version("1.0")
       .about("Automate move Jira issue to github!")
       .author("Stephen Kong.")
       .arg(Arg::with_name("issue")
           .short("i")
           .long("issue")
           .takes_value(true)
           .help("Move issue"))
       .subcommand(SubCommand::with_name("config")
           .about("config credentials")
           .arg(Arg::with_name("jira_email")
                .required(true)
                .takes_value(true)
                .help("Jira email"))
           .arg(Arg::with_name("jira_token")
                .required(true)
                .takes_value(true)
                .help("Jira token")))
       .get_matches();

    if let Some(matches) = matches.subcommand_matches("config") {
        jira_config.email = matches.value_of("jira_email").unwrap().to_string();
        jira_config.api_token = matches.value_of("jira_token").unwrap().to_string();
        confy::store("jira-config", &jira_config);
    }

    if let issue = matches.value_of("issue") {
        match jira::fetch_issue(issue.unwrap(), &jira_config) {
          Ok(content) => { println!("{}", content.replace(" ", "-"));},
          Err(error) => { println!("{:?}", error); }
        }
    }

    println!("{:?}", jira_config);
}



