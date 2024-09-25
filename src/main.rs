use clap::Parser;
use std::net::IpAddr;
use tokio::{net::TcpStream, runtime::Runtime, sync::mpsc};

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
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    assert!(args.port_start > 0);
    assert!(args.port_end >= args.port_start);

    let rt = Runtime::new()?;

    let (tx, mut rx) = mpsc::channel(10);
    // let mut open_ports = vec![];
    rt.block_on(async {
        for port in args.port_start..=args.port_end {
            println!("? {}:{}", args.addr, port);
            let tx = tx.clone();
            let task = tokio::spawn(
                async move{
                    let connection_attempt = TcpStream::connect((args.addr, port)).await;
                    if let Ok(_open) = connection_attempt{
                        tx.send((args.addr, port)).await.unwrap();
                    };
                });
            let _ = task.await;
        }
    });

    while let Ok((addr, port)) = rx.try_recv() {
        println!("= {}:{}", addr, port);
    }
    // println!("{}", args.addr);
    Ok(())
}