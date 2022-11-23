use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Use given profile for logs
    #[arg(short, long)]
    profile: Option<String>,

    /// Print available profiles
    #[arg(short, long)]
    list: bool,
}
