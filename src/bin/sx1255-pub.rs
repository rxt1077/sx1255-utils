use zeromq::prelude::*;
use zeromq::ZmqMessage;
use clap::Parser;
use std::time::{Instant, Duration};
use std::io::Read;
use tokio::time::sleep;
use alsa::{Direction, ValueOr};
use alsa::pcm::{PCM, HwParams, Format, Access};

/// takes IQ baseband samples from SX1255 vi the I2S audio device and puts them
/// on a ZeroMQ pub socket
#[derive(Parser)]
struct Args {
    /// audio device
    #[arg(short, long, default_value="default")]
    device: String,

    /// sample rate for audio device
    #[arg(short, long, default_value="192000")]
    sample_rate: u32,

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

    println!("Opening audio device");
    let pcm = match PCM::new(&args.device, Direction::Capture, false) {
        Ok(pcm) => pcm,
        Err(e) => {
            println!("Error opening audio device {}: {}", &args.device, e);
            return
        },
    };
    println!("Setting audio HW params");
    let hwp = match HwParams::any(&pcm) {
        Ok(hwp) => hwp,
        Err(e) => {
            println!("Unable to get audio default HW params: {}", e);
            return
        },
    };
    match hwp.set_channels(2) {
        Ok(_) => {},
        Err(e) => {
            println!("Unable to set audio channels to 2: {}", e);
            return
        }
    }
    match hwp.set_rate(args.sample_rate, ValueOr::Greater) {
        Ok(_) => {},
        Err(e) => {
            println!("Unable to set audio sample rate to {}: {}", &args.sample_rate, e);
            return
        },
    }
    match hwp.set_format(Format::s16()) {
        Ok(_) => {},
        Err(e) => {
            println!("Unable to set audio format to {}: {}", Format::s16(), e);
            return
        },
    }
    match hwp.set_access(Access::RWInterleaved) {
        Ok(_) => {},
        Err(e) => {
            println!("Unable to set audio access mode to {:?}: {}", Access::RWInterleaved, e);
            return
        },
    }
    match pcm.hw_params(&hwp) {
        Ok(_) => {},
        Err(e) => {
            println!("Unable to set audio HW params: {}", e);
        },
    }
    let mut io = match pcm.io_i16() {
        Ok(io) => io,
        Err(e) => {
            println!("Unable to get IO for audio PCM: {}", e);
            return
        },
    };

    println!("Starting ZeroMQ server");
    let mut socket = zeromq::PubSocket::new();
    match socket.bind(&args.endpoint).await {
        Ok(_) => {},
        Err(e) => {
            println!("Error binding to socket: {}", e);
            return
        },
    }

    println!("Starting sending loop");
    let mut count = 0u32;
    let mut start = Instant::now();
    let mut buf = [0u8; 5000];
    loop {
        match io.read_exact(&mut buf) {
            Ok(_) => {},
            Err(e) => {
                println!("Error reading audio: {}", e);
                return
            },
        }
        let m: ZmqMessage = ZmqMessage::from(buf.to_vec());
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
