

mod credential;

use structopt::StructOpt;
#[derive(StructOpt)]
struct Cli {
    issue: String,
    email: String,
    api_token: String
}

fn main()  {
    let args = Cli::from_args();
    let credit = credential::JiraCredential::from(&args.email, &args.api_token);
    let secret = credit.secret();

    match jiraify::fetch_issue(&args.issue, &secret) {
      Ok(content) => { println!("{}", content.replace(" ", "-"));},
      Err(error) => { println!("{:?}", error); }
    }
}



