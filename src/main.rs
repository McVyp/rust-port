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
    
    rt.block_on(async {
        let mut tasks = vec![];
        for port in args.port_start..=args.port_end {
            println!("? {}:{}", args.addr, port);
            let tx = tx.clone();
            let task = tokio::spawn(
                async move{
                    let scan_attempt = scan(args.addr, port, tx).await;
                    if let Err(err) = scan_attempt{
                        eprintln!("Error: {err}");
                         //or use panic!
                    }
                });
                tasks.push(task);
            }
            for task in tasks {
                task.await.unwrap();
            }
    });

    drop(tx);

    while let Ok((addr, port)) = rx.try_recv() {
        println!("= {}:{}", addr, port);
    }
    // println!("{}", args.addr);
    Ok(())
}

async fn scan(addr: IpAddr, port: u16, results_tx: mpsc::Sender<(IpAddr, u16)>) -> Result<(), mpsc::error::SendError<(IpAddr, u16)>> {
    let connection_attempt = TcpStream::connect((addr, port)).await;
    if let Ok(_open) = connection_attempt{
       results_tx.send((addr, port)).await?;
    };
    Ok(())
}
