mod fetcher;
mod printer;
mod utils;

use clap::{ArgAction, Parser};
use colored::Colorize;
use fetcher::{fetch, FetchOpts};
use printer::{data_lines, space_lines};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, action=ArgAction::SetFalse, default_value_t = true)]
    os: bool,
    #[arg(long, action=ArgAction::SetFalse, default_value_t = true)]
    host: bool,
    #[arg(long, action=ArgAction::SetFalse, default_value_t = true)]
    kernel: bool,
    #[arg(long)]
    user: bool,

    #[arg(long, action=ArgAction::SetFalse, default_value_t = true)]
    uptime: bool,
    #[arg(long, action=ArgAction::SetFalse, default_value_t = true)]
    cpu: bool,
    #[arg(long, action=ArgAction::SetFalse, default_value_t = true)]
    memory: bool,
    #[arg(long, action=ArgAction::SetFalse, default_value_t = true)]
    packages: bool,

    #[arg(long, action)]
    wm: bool,
    #[arg(long, action)]
    session: bool,
    #[arg(long, action)]
    terminal: bool,

    #[arg(long, action)]
    gpu1: bool,
    #[arg(long, action)]
    gpu2: bool,
}

fn main() {
    let args = Args::parse();

    let opts = FetchOpts {
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

    let data = fetch(opts);
    let lines = space_lines(data_lines(data));

    // so that lines don't wrap on small terminals
    crossterm::execute!(std::io::stdout(), crossterm::terminal::DisableLineWrap)
        .unwrap_or_default();

    for (key, value) in lines {
        println!("{} {}", key.blue().bold(), value.white());
    }
}
