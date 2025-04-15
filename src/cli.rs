use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "ReconX", about = "Scanner de portas e DNS Zone Transfer")]

pub struct CliArgs {
    #[clap(short, long)]
    pub target: String,

    #[clap(long, default_value_t = false)]
    pub dns_zone_transfer: bool,

    #[clap(long)]
    pub ports: Option<String>,

    #[clap(long)]
    pub only_open: bool,
}
