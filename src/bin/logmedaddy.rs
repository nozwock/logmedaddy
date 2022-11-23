use clap::Parser;
use logmedaddy::Args;

fn main() {
    let _cli = Args::parse();

    let _cfg = logmedaddy::Config::load();
    dbg!(&_cfg);

    // TODO: ability to pass multiple profiles
    // TODO: obv impl the -p and -l options
    // TODO: auto completion for shell?
}
