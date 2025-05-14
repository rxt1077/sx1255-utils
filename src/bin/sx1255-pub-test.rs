use clap::Parser;
use std::time::Instant;

/// Subscribes to sx1255-pub and periodically prints the received sample rate
#[derive(Parser)]
struct Args {
    /// ZeroMQ endpoint
    #[arg(short, long, default_value="tcp://127.0.0.1:17017")]
    endpoint: String,
}

fn main() {
    let args = Args::parse();
    
    println!("Connecting to server...");
    let context = zmq::Context::new();
    let subscriber = match context.socket(zmq::SUB) {
        Ok(subscriber) => subscriber, 
        Err(e) => {
            println!("Error creating subscriber: {}", e);
            return
        },
    };
    match subscriber.connect(&args.endpoint) {
        Ok(_) => {},
        Err(e) => {
            println!("Error connecting subscriber: {}", e);
            return
        },
    }
    match subscriber.set_subscribe(b"") {
        Ok(_) => {},
        Err(e) => {
            println!("Could not subscribe to all topics: {}", e);
            return
        },
    }
    
    println!("Starting receiving loop...");
    let mut count = 0u32;
    let mut start = Instant::now();
    let mut msg = zmq::Message::new();
    loop {
        match subscriber.recv(&mut msg, 0) {
            Ok(_) => {},
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
