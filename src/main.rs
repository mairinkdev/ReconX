mod cli;
mod scanner;
mod dns_axfr;

use clap::Parser;
use cli::CliArgs;

#[tokio::main]
async fn main() {
    ascii_header();
    let args = CliArgs::parse();

    if args.dns_zone_transfer {
        dns_axfr::attempt_zone_transfer(&args.target).await;
    } else {
        scanner::scan_target(&args.target).await;
    }
}

fn ascii_header(){
    println!("{}", "==================================================");
    println!("{}", "              R   E   C   O   N   X");
    println!("{}", "              https://github.com/mairinkdev");
    println!("{}", "==================================================");
}
