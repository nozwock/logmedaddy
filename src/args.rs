use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
            ArgGroup::new("info")
                .args(["list", "config_path"])
                .conflicts_with_all(["profile"]),
        ))]
pub struct Args {
    /// Use given profile for logs
    #[arg(short, long, required = true)]
    pub profile: Option<String>,

    /// Print available profiles
    #[arg(short, long)]
    pub list: bool,

    /// Print config path
    #[arg(long)]
    pub config_path: bool,
}
