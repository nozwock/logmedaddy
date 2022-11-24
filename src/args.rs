use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Use given profile for logs
    #[arg(short, long, required = true, conflicts_with = "list")]
    pub profile: Option<String>,

    /// Print available profiles
    #[arg(short, long)]
    pub list: bool,
}
