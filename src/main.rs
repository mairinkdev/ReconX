mod cli;
mod scanner;
mod dns_axfr;

use clap::Parser;
use cli::CliArgs;
use std::env;

#[tokio::main]
async fn main() {
    ascii_header();
    println!("🔧 Sistema Operacional detectado: {}", detect_os());
    let args = CliArgs::parse();

    if args.dns_zone_transfer || args.dns_raw {
        dns_axfr::attempt_zone_transfer(&args.target, args.dns_raw).await;
    } else {
        scanner::scan_target(&args.target, &args.ports, args.only_open, &args.output).await;
    }
}

fn ascii_header() {
    println!("{}", "==================================================");
    println!("{}", "              R   E   C   O   N   X");
    println!("{}", "              https://github.com/mairinkdev");
    println!("{}", "==================================================");
}

fn detect_os() -> &'static str {
    match env::consts::OS {
        "windows" => "🪟  Windows",
        "linux" => "🐧  Linux",
        "macos" => "🍎  macOS",
        other => other,
    }
}
