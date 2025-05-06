use zeromq::prelude::*;
use clap::Parser;
use std::time::Instant;

/// Subscribes to sx1255-pub and periodically prints the received sample rate
#[derive(Parser)]
struct Args {
    /// ZeroMQ endpoint
    #[arg(short, long, default_value="tcp://127.0.0.1:17017")]
    endpoint: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    println!("Connecting to server...");
    let mut socket = zeromq::SubSocket::new();
    match socket.connect(&args.endpoint).await {
        Ok(_) => {},
        Err(e) => {
            println!("Error connecting to socket: {}", e);
            return
        },
    }

    match socket.subscribe("").await {
        Ok(_) => {},
        Err(e) => {
            println!("Error subscribing: {}", e);
        },
    }

    println!("Starting receiving loop...");
    let mut count = 0u32;
    let mut start = Instant::now();
    loop {
        let recv = match socket.recv().await {
            Ok(recv) => recv,
            Err(e) => {
                println!("Error receiving: {}", e);
                return
            },
        };
        count += 1;
        let elapsed: u32 = start.elapsed().as_secs() as u32;
        if elapsed >= 10 {
            println!("Receiving {} samples/second", ((5000 / 4) * count) / elapsed);
            count = 0;
            start = Instant::now();
        }
    }
}
