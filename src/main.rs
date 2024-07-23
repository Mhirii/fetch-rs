mod fetcher;
mod logo;
mod printer;
mod utils;

use clap::{ArgAction, Parser};
use fetcher::{fetch, FetchOpts};
use libmacchina::{traits::GeneralReadout as _, GeneralReadout};
use printer::{data_lines, render, space_lines};

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

    #[arg(long, action)]
    all: bool,

    #[arg(long, action=ArgAction::SetFalse, default_value_t = true)]
    logo: bool,
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
        all: Some(args.all),
    };

    let general_readout = GeneralReadout::new();
    let general = &general_readout;

    let data = fetch(general, opts);
    let lines = space_lines(data_lines(data));

    let logo_name = fetcher::fetch_os(general);
    let logo = logo::logo(&logo_name);

    render(lines, logo, args.logo);
}
