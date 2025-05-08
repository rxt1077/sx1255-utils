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
    #[arg(short='r', long, default_value="192000")]
    sample_rate: u32,

    /// sample format for audio device
    #[arg(short='s', long, value_parser=["S16_LE", "S32_LE"], default_value="S16_LE")]
    sample_format: String,

    /// local ZeroMQ endpoint
    #[arg(short, long, default_value="tcp://0.0.0.0:17017")]
    endpoint: String,

    /// message size in bytes (must be a multiple of SAMPLE_SIZE * 2)
    #[arg(short, long, default_value_t=5000)]
    msg_size: usize,

    /// print the rate at which we're publishing samples every 10 seconds
    #[arg(short, long)]
    print_sample_rate: bool,
}

fn main() {
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
    let mut io = pcm.io_bytes();

    println!("Starting ZeroMQ server");
    let context = zmq::Context::new();
    let publisher = match context.socket(zmq::PUB) {
        Ok(publisher) => publisher,
        Err(e) => {
            println!("Error getting socket: {}", e);
            return
        },
    };
    match publisher.bind(&args.endpoint) {
        Ok(_) => {},
        Err(e) => {
            println!("Failed binding publisher: {}", e);
            return
        },
    }

    println!("Starting sending loop");
    let mut start = Instant::now();
    let mut bytes: usize = 0;
    loop {
        let mut buf = vec![0u8; args.msg_size];
        bytes += match io.read(&mut buf) {
            Ok(bytes_read) => {
                if bytes_read != args.msg_size {
                    println!("Bytes read not {}: {}", args.msg_size, bytes_read);
                    return
                }
                bytes_read
            },
            Err(e) => {
                println!("Error reading audio: {}", e);
                return
            },
        };

        match publisher.send(buf, zmq::DONTWAIT) {
            Ok(_) => {},
            Err(e) => {
                println!("Error sending: {}", e);
                return
            },
        }

        if args.print_sample_rate {
            let elapsed: usize = start.elapsed().as_secs() as usize;
            if elapsed >= 10 {
                match args.sample_format.as_str() {
                    "S16_LE" => { println!("{} samples/second", bytes/4/elapsed); }
                    "S32_LE" => { println!("{} samples/second", bytes/8/elapsed); }
                           _ => { println!("{} bytes in {} seconds", bytes, elapsed); }
                }
                bytes = 0;
                start = Instant::now();
            }
        }
    }
}
