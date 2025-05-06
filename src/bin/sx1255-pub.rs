use zeromq::prelude::*;
use zeromq::ZmqMessage;
use clap::Parser;
use std::time::Instant;
use std::io::Read;
use alsa::{Direction, ValueOr};
use alsa::pcm::{PCM, HwParams, Format, Access};

/// takes IQ baseband samples from SX1255 vi the I2S audio device and puts them
/// on a ZeroMQ pub socket
#[derive(Parser)]
struct Args {
    /// audio device (run `arecord -l` to see what's available)
    #[arg(short, long, default_value="hw:1,1")]
    device: String,

    /// sample rate for audio device
    #[arg(short='r', long, default_value="125000")]
    sample_rate: u32,

    /// sample format for audio device
    #[arg(short='s', long, value_parser=["S16_LE", "S32_LE"], default_value="S16_LE")]
    sample_format: String,

    /// local ZeroMQ endpoint
    #[arg(short, long, default_value="tcp://0.0.0.0:17017")]
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
    match hwp.set_rate(args.sample_rate, ValueOr::Nearest) {
        Ok(_) => {},
        Err(e) => {
            println!("Unable to set audio sample rate to {}: {}", &args.sample_rate, e);
            return
        },
    }
    let format = match args.sample_format.as_str() {
        "S16_LE" => Format::s16(),
        "S32_LE" => Format::s32(),
        _ => {
            println!("Invalid audio format");
            return
        },
    };
    match hwp.set_format(format) {
        Ok(_) => {},
        Err(e) => {
            println!("Unable to set audio format to {}: {}", format, e);
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
    let mut start = Instant::now();
//    let mut buf = [0i16; 2500];
    let mut frames: usize = 0;
    loop {
        let mut buf = vec![0u8; 5000];
        frames += match io.read(&mut buf) {
            Ok(frames) => {
                if frames != 5000 {
                    println!("Frame size not 5000: {}", frames);
                    return
                }
                frames
            },
            Err(e) => {
                println!("Error reading audio: {}", e);
                return
            },
        };
        let m: ZmqMessage = ZmqMessage::from(buf);

        match socket.send(m).await {
            Ok(_) => {},
            Err(e) => {
                println!("Error sending: {}", e);
                return
            },
        }

        if args.print_sample_rate {
            let elapsed: u32 = start.elapsed().as_secs() as u32;
            if elapsed >= 10 {
                println!("{} frames in {} seconds", frames, elapsed);
                frames = 0;
                start = Instant::now();
            }
        }
    }
}
