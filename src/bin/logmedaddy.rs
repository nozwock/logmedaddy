use clap::Parser;
use std::{fs::File, io::Write, path::Path, process::exit};

use logmedaddy::{defines, log_profiles, Args};

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();
    let cfg = logmedaddy::Config::load();

    let log_path = if let Some(out) = &cli.output {
        out
    } else {
        "logmedaddy.log"
    };
    let log_path = Path::new(log_path);

    if cli.list {
        println!(
            "Available profiles:\n{}",
            cfg.profiles
                .iter()
                .map(|profile| profile.name.as_str())
                .collect::<Vec<_>>()
                .join("\n")
        );
    };

    if cli.config_path {
        println!("{}", defines::app_config_path().to_string_lossy());
    };

    if cli.all {
        File::create(log_path)?
            .write_all(log_profiles(cfg.profiles.iter().collect::<Vec<_>>()).as_bytes())?;
        println!("Log successfully saved at {:?}", log_path);
    }

    if let Some(profile) = &cli.profile {
        match cfg.profiles.iter().find(|&x| &x.name == profile) {
            Some(profile) => {
                let log = log_profiles(vec![profile]);
                File::create(log_path)?.write_all(log.as_bytes())?;
                println!("Log successfully saved at {:?}", log_path);
            }
            None => {
                println!("No such profile exists!");
                exit(1);
            }
        };
    };

    // // TODO: add an -o option
    // // TODO: obv impl the -p and -l options
    // // TODO: --config_path flag to print config path
    // TODO: ability to pass multiple profiles
    // TODO: auto completion for shell?

    Ok(())
}
