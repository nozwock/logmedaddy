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
    /// Use the given profiles for log
    #[arg(short, long)]
    pub profile: Vec<String>,

    /// Use all available profiles for log
    #[arg(short, long)]
    pub all: bool,

    /// Set output log file name
    #[arg(short, long, requires = "log", value_name = "FILE")]
    pub output: Option<String>,

    /// List available profiles
    #[arg(short, long)]
    pub list: bool,

    /// Force panic on any log command failure
    #[arg(long, requires = "log")]
    pub panic: bool,

    /// Print config path
    #[arg(long)]
    pub config_path: bool,
}
