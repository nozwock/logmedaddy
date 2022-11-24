use clap::Parser;
use logmedaddy::Args;

fn main() {
    let _cli = Args::parse();

    let _cfg = logmedaddy::Config::load();
    dbg!(&_cfg);

    // mock tests
    use std::io::Write;
    let log = logmedaddy::log_profiles(_cfg.profiles);
    std::fs::File::create("./logmedaddy.log")
        .unwrap()
        .write_all(log.as_bytes())
        .unwrap();

    // TODO: ability to pass multiple profiles
    // TODO: obv impl the -p and -l options
    // TODO: --config_path flag to print config path
    // TODO: auto completion for shell?
}
