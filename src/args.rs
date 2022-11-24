use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
#[command(group(
            ArgGroup::new("info")
                .args(["list", "config_path"])
                .conflicts_with_all(["log"]),
        ))]
#[command(group(
            ArgGroup::new("log")
                .args(["profile", "all"])
        ))]
pub struct Args {
    /// Use given profile for logs
    #[arg(short, long)]
    pub profile: Option<String>,

    /// Use all available profiles for logs
    #[arg(short, long)]
    pub all: bool,

    /// Print available profiles
    #[arg(short, long)]
    pub list: bool,

    /// Print config path
    #[arg(long)]
    pub config_path: bool,
}
