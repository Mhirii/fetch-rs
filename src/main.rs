use std::collections::HashSet;

use clap::Parser;
mod fetcher;
mod printer;
mod utils;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = true)]
    os: bool,
    #[arg(long, default_value_t = true)]
    host: bool,
    #[arg(long, default_value_t = true)]
    kernel: bool,
    #[arg(long, default_value_t = true)]
    user: bool,

    #[arg(long, default_value_t = true)]
    uptime: bool,
    #[arg(long, default_value_t = true)]
    cpu: bool,
    #[arg(long, default_value_t = true)]
    memory: bool,
    #[arg(long, default_value_t = true)]
    packages: bool,

    #[arg(long, default_value_t = true)]
    wm: bool,
    #[arg(long, default_value_t = true)]
    session: bool,
    #[arg(long, default_value_t = true)]
    terminal: bool,

    #[arg(long, default_value_t = true)]
    gpu1: bool,
    #[arg(long, default_value_t = true)]
    gpu2: bool,
}

fn main() {
    use std::time::Instant;
    let start = Instant::now();

    let args = Args::parse();

    let opts = fetcher::FetchOpts {
        os: Some(args.os),
        host: Some(args.host),
        kernel: Some(args.kernel),
        user: Some(args.user),
        uptime: Some(args.uptime),
        cpu: Some(args.cpu),
        memory: Some(args.memory),
        window_manager: Some(args.wm),
        session: Some(args.session),
        terminal: Some(args.terminal),
        packages: Some(args.packages),
        gpu1: Some(args.gpu1),
        gpu2: Some(args.gpu2),
    };

    let data = fetcher::fetch(opts);
    let lines = printer::data_lines(data);
    for (key, value) in lines {
        println!("{}: {}", key, value);
    }

    let duration = start.elapsed();

    println!("Time elapsed in main() is: {:?}", duration);
}
