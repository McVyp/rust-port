use clap::Parser;
use std::net::IpAddr;

#[derive(Debug, Parser)]
struct Args {
    #[arg()]
    addr: IpAddr,
}
fn main() {
    let args = Args::parse();
    println!("{}", args.addr);
}