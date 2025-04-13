use std::io;
use clap::{Parser, Subcommand};
use spidev::{Spidev, SpidevOptions, SpiModeFlags};
use std::path::PathBuf;

use crate::info::{SX1255Info, get_info, print_info, set_info};
use crate::file::{write_file};

pub mod info;
pub mod file;

static SPI_DEV: &str = "/dev/spidev0.0";
static SPI_OPTS: SpidevOptions = SpidevOptions {
    bits_per_word: Some(8),
    max_speed_hz: Some(500000),
    lsb_first: Some(false),
    spi_mode: Some(SpiModeFlags::SPI_MODE_0),
};

#[derive(Parser)]
#[command(name = "sx1255-config")]
#[command(version)]
#[command(about = "Configure the M17 sx1255 HAT via SPI/GPIO")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Prints info about device state
    Info,
    /// Resets the device
    Reset,
    /// Save device state to file
    Save {
        /// file name
        #[arg(short, long)]
        file: PathBuf,
    },
    /// Loads device state from file
    Load {
        /// file name
        #[arg(short, long)]
        file: PathBuf,
    },
    /// Sets a register variable
    Set {
        /// register variable name
        #[command(subcommand)]
        name: SetCommands,
    },
}

#[derive(Subcommand)]
#[command(rename_all = "snake_case")]
enum SetCommands {
    /// Enables the PA driver
    DriverEnable{
        #[arg(value_parser=["true", "false"])]
        value: String,
    },
    /// Enables the complete Tx part of the front-end (except the PA)
    TxEnable {
        #[arg(value_parser=["true", "false"])]
        value: String,
    },
    /// Enables the complete Rx part of the front-end
    RxEnable {
        #[arg(value_parser=["true", "false"])]
        value: String,
    },
    /// Enables the PDS and the oscillator
    RefEnable {
        #[arg(value_parser=["true", "false"])]
        value: String,
    },
    /// Sets the Rx frequency
    RxFreq {
        #[arg(value_parser=clap::value_parser!(u32).range(300000000..500000000))]
        freq: u32,
    },
    /// Sets the Tx frequency
    TxFreq {
        #[arg(value_parser=clap::value_parser!(u32).range(300000000..500000000))]
        freq: u32,
    },
}

fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open(SPI_DEV)?;
    spi.configure(&SPI_OPTS)?;
    Ok(spi)
}

fn main() {
    let mut spi = create_spi().expect("SPI initialization");

    let cli = Cli::parse();
    let mut sx1255_info = SX1255Info::default();
    get_info(&mut spi, &mut sx1255_info);

    match &cli.command {
        Commands::Info => {
            print_info(sx1255_info);
        },
        Commands::Save  { file } => {
            println!("Saving to {}", file.display());
            write_file(sx1255_info, file).expect("file write");
        },
        Commands::Load { file } => {
            println!("Loading from {}", file.display());
        },
        Commands::Reset => {
            println!("Resetting");
        },
        Commands::Set { name } => {
            match name {
                SetCommands::DriverEnable { value } => {
                    println!("Setting driver_enable to {}", value);
                    match value.as_str() {
                        "true"  => sx1255_info.driver_enable = true,
                        "false" => sx1255_info.driver_enable = false,
                        _       => panic!("Invalid input"),
                    }
                    set_info(&mut spi, sx1255_info);
                },
                SetCommands::TxEnable { value } => {
                    println!("Setting tx_enable to {}", value);
                    match value.as_str() {
                        "true"  => sx1255_info.tx_enable = true,
                        "false" => sx1255_info.tx_enable = false,
                        _       => panic!("Invalid input"),
                    }
                    set_info(&mut spi, sx1255_info);
                },
                SetCommands::RxEnable { value } => {
                    println!("Setting rx_enable to {}", value);
                    match value.as_str() {
                        "true"  => sx1255_info.rx_enable = true,
                        "false" => sx1255_info.rx_enable = false,
                        _       => panic!("Invalid input"),
                    }
                    set_info(&mut spi, sx1255_info);
                },
                SetCommands::RefEnable { value } => {
                    println!("Setting ref_enable to {}", value);
                    match value.as_str() {
                        "true"  => sx1255_info.ref_enable = true,
                        "false" => sx1255_info.ref_enable = false,
                        _       => panic!("Invalid input"),
                    }
                    set_info(&mut spi, sx1255_info);
                },
                SetCommands::RxFreq { freq } => {
                    println!("Setting Rx frequency to {}", *freq);
                    sx1255_info.rx_freq = *freq;
                    set_info(&mut spi, sx1255_info);
                },
                SetCommands::TxFreq { freq } => {
                    println!("Setting Tx frequency to {}", *freq);
                    sx1255_info.tx_freq = *freq;
                    set_info(&mut spi, sx1255_info);
                },
            }
        },
    }
}
