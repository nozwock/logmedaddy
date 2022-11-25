use clap::Parser;
use colored::*;
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
        let space = " ".repeat(4);
        println!(
            "Available profiles:\n{space}{}",
            cfg.profiles
                .iter()
                .map(|profile| profile.name.as_str())
                .collect::<Vec<_>>()
                .join(&format!("\n{space}"))
        );
    };

    if cli.config_path {
        println!("{}", defines::app_config_path().to_string_lossy());
    };

    if cli.all {
        let log = log_profiles(cfg.profiles.iter().collect::<Vec<_>>());
        File::create(log_path)?.write_all(log.as_bytes())?;
        println!("{} {:?}", "Log successfully saved at".green(), log_path);
    }

    if !cli.profile.is_empty() {
        let filtered_profiles = cfg
            .profiles
            .iter()
            .filter(|prof| cli.profile.contains(&prof.name))
            .collect::<Vec<_>>();

        if filtered_profiles.len() != cli.profile.len() {
            let filtered_profile_names = filtered_profiles
                .iter()
                .map(|prof| &prof.name)
                .collect::<Vec<_>>();
            for cli_profile in &cli.profile {
                if !filtered_profile_names.contains(&cli_profile) {
                    println!("{} '{}'", "Profile not found:".red(), cli_profile.yellow());
                }
            }
            exit(69);
        };

        let log = log_profiles(filtered_profiles);
        File::create(log_path)?.write_all(log.as_bytes())?;
        println!("{} {:?}", "Log successfully saved at".green(), log_path);
    };

    // TODO?: number profiles; mention profiles by number
    // TODO: option to panic upon log fail
    // TODO: option to split the output log file by profile names
    // // TODO: ability to pass multiple profiles (DONE)
    // TODO: auto completion for shell?

    Ok(())
}
