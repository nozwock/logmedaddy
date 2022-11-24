use clap::Parser;
use logmedaddy::Args;

fn main() {
    let cli = Args::parse();
    let cfg = logmedaddy::Config::load();

    match &cli.list {
        true => {
            println!(
                "Available profiles:\n{}",
                cfg.profiles
                    .iter()
                    .map(|profile| profile.name.as_str())
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
        false => {}
    };

    match &cli.profile {
        Some(profile) => {
            use std::io::Write;
            let log_path = "./logmedaddy.log";
            let log = logmedaddy::log_profiles(vec![cfg
                .profiles
                .iter()
                .find(|&x| &x.name == profile)
                .unwrap()]);
            std::fs::File::create(log_path)
                .unwrap()
                .write_all(log.as_bytes())
                .unwrap();
            println!("Log successfully saved at {:?}", log_path);
        }
        None => {}
    };

    // TODO: ability to pass multiple profiles
    // TODO: obv impl the -p and -l options
    // TODO: --config_path flag to print config path
    // TODO: auto completion for shell?
}
