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

static REG_MODE: u8    = 0x00;
static REG_FRFH_RX: u8 = 0x01;
static REG_FRFM_RX: u8 = 0x02;
static REG_FRFL_RX: u8 = 0x03;
static REG_FRFH_TX: u8 = 0x04;
static REG_FRFM_TX: u8 = 0x05;
static REG_FRFL_TX: u8 = 0x06;
static REG_VERSION: u8 = 0x07;
static REG_TXFE1: u8   = 0x08;
static REG_TXFE2: u8   = 0x09;
static REG_TXFE3: u8   = 0x0A;
static REG_TXFE4: u8   = 0x0B;

struct SX1255Info {
    driver_enable: bool,
    tx_enable: bool,
    rx_enable: bool,
    ref_enable: bool,
    rx_freq: u32,
    tx_freq: u32,
    version: u8,
    tx_dac_gain: &'static str,
    tx_mixer_gain: f64,
    tx_mixer_tank_cap: u32,
    tx_mixer_tank_res: f64,
    tx_pll_bw: u32,
    tx_filter_bw: f64,
}

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
    let tx_buf = [addr, 0];
    let mut rx_buf = [0_u8; 2];
    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        spi.transfer(&mut transfer)?;
    }
    Ok(rx_buf[1])
}

fn freq_to_u32(frfh: u8, frfm: u8, frfl: u8) -> u32 {
    (((((frfh as u32) << 16) +
       ((frfm as u32) << 8) +
       ((frfl as u32) << 0)) as f64)
     * (32000000.0 / 1048576.0)) as u32
}

fn tx_dac_gain_to_str(gain: u8) -> &'static str {
    match gain {
        0b000 => "maximum gain - 9 dB",
        0b001 => "maximum gain - 6 dB",
        0b010 => "maximum gain - 3 dB",
        0b011 => "maximum gain (0 dB full scale)",
        0b100 => "Max gain - 9 dB with test Vref voltage",
        0b101 => "Max gain - 6 dB with test Vref voltage",
        0b110 => "Max gain - 3 dB with test Vref voltage",
        0b111 => "Max gain, 0 dBFS with test Vref voltage",
        _     => "unknown",
    }
}

fn tx_mixer_tank_res_to_float(res: u8) -> f64 {
    match res {
        0b000 => 0.95,
        0b001 => 1.11,
        0b010 => 1.32,
        0b011 => 1.65,
        0b100 => 2.18,
        0b101 => 3.24,
        0b110 => 6.00,
        0b111 => 64.0,
        _     => -1.0,
    }
}

fn main() {
    let mut spi = create_spi().expect("SPI initialization");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Info => {
            let mode = sx1255_readreg(&mut spi, REG_MODE).expect("read mode register");
            let frfh_rx = sx1255_readreg(&mut spi, REG_FRFH_RX).expect("read FRFH_RX register");
            let frfm_rx = sx1255_readreg(&mut spi, REG_FRFM_RX).expect("read FRFM_RX register");
            let frfl_rx = sx1255_readreg(&mut spi, REG_FRFL_RX).expect("read FRFL_RX register");
            let frfh_tx = sx1255_readreg(&mut spi, REG_FRFH_TX).expect("read FRFH_TX register");
            let frfm_tx = sx1255_readreg(&mut spi, REG_FRFM_TX).expect("read FRFM_TX register");
            let frfl_tx = sx1255_readreg(&mut spi, REG_FRFL_TX).expect("read FRFL_TX register");
            let version = sx1255_readreg(&mut spi, REG_VERSION).expect("read version register");
            let txfe1 = sx1255_readreg(&mut spi, REG_TXFE1).expect("read TXFE1 register");
            let txfe2 = sx1255_readreg(&mut spi, REG_TXFE2).expect("read TXFE2 register");
            let txfe3 = sx1255_readreg(&mut spi, REG_TXFE3).expect("read TXFE3 register");
            let txfe4 = sx1255_readreg(&mut spi, REG_TXFE4).expect("read TXFE4 register");

            let info = SX1255Info {
                driver_enable: (mode & 0b00001000) != 0,
                tx_enable:     (mode & 0b00000100) != 0,
                rx_enable:     (mode & 0b00000010) != 0,
                ref_enable:    (mode & 0b00000001) != 0,
                rx_freq: freq_to_u32(frfh_rx, frfm_rx, frfl_rx),
                tx_freq: freq_to_u32(frfh_tx, frfm_tx, frfl_tx),
                version: version,
                tx_dac_gain: tx_dac_gain_to_str((txfe1 & 0b01110000) >> 4),
                tx_mixer_gain: -37.5 + ((2 * (txfe1 & 0b00001111)) as f64),
                tx_mixer_tank_cap: 128 * (((txfe2 & 0b00111000) >> 3) as u32),
                tx_mixer_tank_res: tx_mixer_tank_res_to_float(txfe2 & 0b00000111),
                tx_pll_bw: (((((txfe3 & 0b01100000) >> 4) as u32) + 1) * 75),
                tx_filter_bw: 17.15 / ((41 - (txfe3 & 0b00001111)) as f64),
            };

            println!("         PA driver enabled: {}", info.driver_enable);
            println!("                Tx enabled: {}", info.tx_enable);
            println!("                Rx enabled: {}", info.rx_enable);
            println!("PDS and oscillator enabled: {}", info.ref_enable);
            println!("      Rx carrier frequency: {} Hz", info.rx_freq);
            println!("      Tx carrier frequency: {} Hz", info.tx_freq);
            println!("              Version code: 0x{:02X}", info.version);
            println!("               Tx DAC gain: {}", info.tx_dac_gain);
            println!("             Tx mixer gain: {} dB", info.tx_mixer_gain);
            println!(" Tx mixer tank capacitance: {} fF", info.tx_mixer_tank_cap);
            println!("  Tx mixer tank resistance: {} kΩ", info.tx_mixer_tank_res);
            println!("          Tx PLL bandwidth: {} KHz", info.tx_pll_bw);
            println!("       Tx filter bandwidth: {:.6} MHz", info.tx_filter_bw);
        },
        Commands::Reset => {
            println!("Reset");
        },
    }
}
