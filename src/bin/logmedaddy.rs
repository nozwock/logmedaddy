use clap::Parser;
use colored::*;
use std::{fs::File, io::Write, path::Path, process::exit};

use logmedaddy::{defines, Args, Logger};

#[derive(Debug)]
enum OutputType<'a> {
    File(&'a Path),
    Stdout,
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();
    let cfg = logmedaddy::Config::load();

    let output_type = if let Some(out) = &cli.output {
        OutputType::File(Path::new(out))
    } else {
        OutputType::Stdout
    };

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
        let log = Logger::from_profiles(&cfg.profiles);
        let log = if cli.panic {
            if let Some(output) = log.to_string() {
                output
            } else {
                exit(42);
            }
        } else {
            log.to_string_lossy()
        };
        match output_type {
            OutputType::File(path) => {
                File::create(path)?.write_all(log.as_bytes())?;
                println!("{} {:?}", "Log successfully saved at".green(), path);
            }
            OutputType::Stdout => print!("{}", log),
        };
    };

    if !cli.profile.is_empty() {
        let filtered_profiles = cfg
            .profiles
            .into_iter()
            .filter(|prof| cli.profile.contains(&prof.name))
            .collect::<Vec<_>>();

        if filtered_profiles.len() != cli.profile.len() {
            let filtered_profile_names = filtered_profiles
                .iter()
                .map(|prof| &prof.name)
                .collect::<Vec<_>>();
            for cli_profile in &cli.profile {
                if !filtered_profile_names.contains(&cli_profile) {
                    eprintln!("{} '{}'", "Profile not found:".red(), cli_profile.yellow());
                }
            }
            exit(69);
        };

        let log = Logger::from_profiles(&filtered_profiles);
        let log = if cli.panic {
            if let Some(output) = log.to_string() {
                output
            } else {
                exit(42);
            }
        } else {
            log.to_string_lossy()
        };
        match output_type {
            OutputType::File(path) => {
                File::create(path)?.write_all(log.as_bytes())?;
                println!("{} {:?}", "Log successfully saved at".green(), path);
            }
            OutputType::Stdout => print!("{}", log),
        };
    };

    // TODO*: add more default loggers/profiles
    // TODO?: option to split the output log file by profile names
    // TODO?: numbered profiles; can mention profiles by their number
    // TODO?: auto completion for shell?

    Ok(())
}
