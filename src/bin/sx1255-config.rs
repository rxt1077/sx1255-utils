use std::io;
use clap::{Parser, Subcommand};
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SpiModeFlags};

static SPI_DEV: &str = "/dev/spidev0.0";
static SPI_OPTS: SpidevOptions = SpidevOptions {
    bits_per_word: Some(8),
    max_speed_hz: Some(500000),
    lsb_first: Some(false),
    spi_mode: Some(SpiModeFlags::SPI_MODE_0),
};
static REG_VERSION: u8 = 0x07;

#[derive(Parser)]
#[command(name = "sx1255-config")]
#[command(version)]
#[command(about = "Configure the m17 sx1255 HAT via SPI")]
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
}

fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open(SPI_DEV)?;
    spi.configure(&SPI_OPTS)?;
    Ok(spi)
}

fn sx1255_readreg(spi: &mut Spidev, addr: u8) -> io::Result<u8> {
    let tx_buf = [addr];
    let mut rx_buf = [0_u8; 1];
    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        spi.transfer(&mut transfer)?;
    }
    Ok(rx_buf[0])
}


fn main() {
    let mut spi = create_spi().expect("SPI initialization");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Info => {
            println!("Info");
            println!("chip_version: 0x{:x}", sx1255_readreg(&mut spi, REG_VERSION).expect("read version register"));
        },
        Commands::Reset => {
            println!("Reset");
        },
    }
}
