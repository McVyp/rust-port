use clap::Parser;
use std::net::IpAddr;

#[derive(Debug, Parser)]
struct Args {
    #[arg()]
    addr: IpAddr,

    /// --port_start 1
    #[arg(long, default_value_t = 1)]
    port_start: u16,

    #[arg(long, default_value_t = 1024)]
    port_end: u16
}
fn main() {
    let args = Args::parse();
    assert!(args.port_start > 0);
    assert!(args.port_end >= args.port_start);
    println!("{}", args.addr);
}