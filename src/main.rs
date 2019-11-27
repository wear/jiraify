use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    issue_id: u32
}

fn main() {
    let args = Cli::from_args();
    println!("{}", args.issue_id);
}
