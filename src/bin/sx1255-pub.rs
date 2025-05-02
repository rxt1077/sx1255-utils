use zeromq::prelude::*;
use zeromq::ZmqMessage;
use clap::Parser;
use std::time::{Instant, Duration};
use tokio::time::sleep;

/// takes IQ baseband samples from SX1255 vi the I2S audio device and puts them
/// on a ZeroMQ pub socket
#[derive(Parser)]
struct Args {
    /// local ZeroMQ endpoint
    #[arg(short, long, default_value="tcp://*:17017")]
    endpoint: String,

    /// message size in bytes (must be a multiple of SAMPLE_SIZE * 2)
    #[arg(short, long, default_value_t=5000)]
    msg_size: u32,

    /// print the rate at which we're publishing samples every 10 seconds
    #[arg(short, long)]
    print_sample_rate: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    println!("Starting server...");
    let mut socket = zeromq::PubSocket::new();
    match socket.bind(&args.endpoint).await {
        Ok(_) => {},
        Err(e) => {
            println!("Error binding to socket: {}", e);
            return
        },
    }

    println!("Starting sending loop...");
    let mut count = 0u32;
    let mut start = Instant::now();
    loop {
        // TODO: read 5000 bytes from the audio source
        let m: ZmqMessage = ZmqMessage::from(vec![0_u8; 5000]);
        sleep(Duration::from_millis(100)).await;

        match socket.send(m).await {
            Ok(_) => {},
            Err(e) => {
                println!("Error sending: {}", e);
                return
            },
        }

        if args.print_sample_rate {
            count += 1;
            let elapsed: u32 = start.elapsed().as_secs() as u32;
            if elapsed >= 10 {
                println!("{} samples/second", ((5000 / 4) * count) / elapsed);
                count = 0;
                start = Instant::now();
            }
        }
    }
}
