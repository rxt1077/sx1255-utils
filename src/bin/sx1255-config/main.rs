use std::io;
use clap::{Parser, Subcommand};
use spidev::{Spidev, SpidevOptions, SpiModeFlags};
use std::path::PathBuf;

use crate::info::{SX1255Info, read_info, print_info};

pub mod info;

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
    read_info(&mut spi, &mut sx1255_info);

    match &cli.command {
        Commands::Info => {
            print_info(sx1255_info);
        },
        Commands::Save  { file } => {
            println!("Saving to {}", file.display());
        },
        Commands::Load { file } => {
            println!("Loading from {}", file.display());
        },
        Commands::Reset => {
            println!("Reset");
        },
        Commands::Set { name } => {
            match name {
                SetCommands::DriverEnable { value } => {
                    println!("Setting driver_enable to {}", value);
                },
                SetCommands::TxEnable { value } => {
                    println!("Setting tx_enable to {}", value);
                },
                SetCommands::RxEnable { value } => {
                    println!("Setting rx_enable to {}", value);
                },
                SetCommands::RefEnable { value } => {
                    println!("Setting ref_enable to {}", value);
                },
            }
        },
    }
}
