use std::io;
use clap::{Parser, Subcommand};
use spidev::{Spidev, SpidevOptions, SpiModeFlags};
use std::path::PathBuf;

use crate::info::{SX1255Info, get_info, print_info, set_info};
use crate::file::{write_file, read_file};
use crate::opts::OPTS;

pub mod info;
pub mod file;
pub mod opts;

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
        #[arg()]
        file: PathBuf,
    },
    /// Loads device state from file
    Load {
        /// file name
        #[arg()]
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
        #[arg(action=clap::ArgAction::Set)]
        value: bool,
    },
    /// Enables the complete Tx part of the front-end (except the PA)
    TxEnable {
        #[arg(action=clap::ArgAction::Set)]
        value: bool,
    },
    /// Enables the complete Rx part of the front-end
    RxEnable {
        #[arg(action=clap::ArgAction::Set)]
        value: bool,
    },
    /// Enables the PDS and the oscillator
    RefEnable {
        #[arg(action=clap::ArgAction::Set)]
        value: bool,
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
    /// Sets the Tx DAC gain 
    TxDacGain {
        /// 0: max gain - 9 dB
        /// 1: max gain - 6 dB
        /// 2: max gain - 3 dB
        /// 3: max gain - 0 dB
        /// test modes not recommended
        /// 4: max gain - 9 dB with test Vref voltage
        /// 5: max gain - 6 dB with test Vref voltage
        /// 6: max gain - 3 dB with test Vref voltage
        /// 7: max gain - 0 dB with test Vref voltage
        #[arg(verbatim_doc_comment, value_parser=clap::value_parser!(u8).range(0..8))]
        gain: u8,
    },
    /// Sets the Tx mixer gain
    TxMixerGain {
        /// Gain ~ -37.5 + 2 x GAIN in dB (GAIN must be between 0-15)
        #[arg(value_parser=clap::value_parser!(u8).range(0..16))]
        gain: u8,
    },
    /// Sets the capacitance in parallel with the mixer tank
    TxMixerTankCap {
        /// Capacitance = 128 * CAP fF (CAP must be between 0-7)
        #[arg(value_parser=clap::value_parser!(u8).range(0..8))]
        cap: u8,
    },
    /// Sets the resistance inparallel with the mixer tank
    TxMixerTankRes {
        /// 0: 0.95 kΩ
        /// 1: 1.11 kΩ
        /// 2: 1.32 kΩ
        /// 3: 1.65 kΩ
        /// 4: 2.18 kΩ
        /// 5: 3.24 kΩ
        /// 6: 6.00 kΩ
        /// 7: none => about 64 kΩ
        #[arg(value_parser=clap::value_parser!(u8).range(0..8))]
        res: u8,
    },
    /// Sets Tx PLL bandwidth
    TxPllBw {
        /// PLL Bandwidth = (BW + 1) * 75 KHz (BW must be between 0-3)
        #[arg(value_parser=clap::value_parser!(u8).range(0..4))]
        bw: u8,
    },
    /// Sets Tx analog filter bandwidth DSB
    TxFilterBw {
        /// BW_3dB = 17.15 / (41 - BW) MHz (BW must be between 0-16)
        #[arg(value_parser=clap::value_parser!(u8).range(0..16))]
        bw: u8,
    },
    /// Sets the number of taps for the Tx FIR-DAC
    TxDacBw {
        /// Actual number of taps = 24 + 8 * BW, max = 64
        #[arg(value_parser=clap::value_parser!(u8).range(0..8))]
        bw: u8,
    },
    /// Sets the Rx LNA gain
    RxLnaGain {
        /// 0 = not used
        /// 1 = G1 - highest gain power - 0 dB
        /// 2 = G2 - highest gain power - 6 dB
        /// 3 = G3 - highest gain power - 12 dB
        /// 4 = G4 - highest gain power - 24 dB
        /// 5 = G5 - highest gain power - 36 dB
        /// 6 = G6 - highest gain power - 48 dB
        /// 7 = not used
        #[arg(verbatim_doc_comment, value_parser=clap::value_parser!(u8).range(0..8))]
        gain: u8,
    },
    /// Sets the Rx PGA gain
    RxPgaGain {
        /// lowest gain + 2 dB * GAIN (GAIN must be between 0-15)
        #[arg(value_parser=clap::value_parser!(u8).range(0..16))]
        gain: u8,
    },
    /// Sets the Rx input impedance
    RxZin200 {
        /// 0=50Ω, 1=200Ω
        #[arg(value_parser=clap::value_parser!(u8).range(0..2))]
        imp: u8,
    },
    /// Sets the Rx ADC trim for 36 MHz reference crystal (must be between 0-7)
    RxAdcTrim {
        #[arg(value_parser=clap::value_parser!(u8).range(0..8))]
        trim: u8,
    },
    /// Sets the Rx analog roofing filter
    RxPgaBw {
        /// 0=1500 kHz, 1=1000 kHz, 2=750 kHz, 3=500 kHz
        #[arg(value_parser=clap::value_parser!(u8).range(0..4))]
        bw: u8,
    },
    /// Sets the Rx PLL bandwidth
    RxPllBw {
        /// PLL BW = (BW + 1) * 75 KHz (BW must be between 0-3)
        #[arg(value_parser=clap::value_parser!(u8).range(0..4))]
        bw: u8,
    },
    /// Puts the Rx ADC into temperature measurement mode
    RxAdcTemp {
        #[arg(action=clap::ArgAction::Set)]
        value: bool,
    },
}

fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open(SPI_DEV)?;
    spi.configure(&SPI_OPTS)?;
    Ok(spi)
}

fn main() {
    /* let mut spi = match create_spi() {
        Ok(spi) => spi,
        Err(e) => {
            println!("Unable to open SPI: {}", e);
            return
        },
    }; */

    let cli = Cli::parse();
    let mut sx1255_info = SX1255Info::default();
    //get_info(&mut spi, &mut sx1255_info);

    match &cli.command {
        Commands::Info => {
            print_info(sx1255_info);
        },
        Commands::Save  { file } => {
            println!("Saving to {}", file.display());
            match write_file(sx1255_info, file) {
                Ok(_) => {},
                Err(e) => {
                    println!("Error writing to {}: {}", file.display(), e);
                    return
                },
            };
        },
        Commands::Load { file } => {
            println!("Loading from {}", file.display());
            match read_file(&mut sx1255_info, file) {
                Ok(_) => {},
                Err(e) => {
                    println!("Error loading from {}: {}", file.display(), e);
                    return
                },
            };
        },
        Commands::Reset => {
            println!("Resetting");
        },
        Commands::Set { name } => {
            match name {
                SetCommands::DriverEnable { value } => {
                    println!("Setting driver_enable to {}", value);
                    sx1255_info.driver_enable = *value;
                },
                SetCommands::TxEnable { value } => {
                    println!("Setting tx_enable to {}", value);
                    sx1255_info.tx_enable = *value;
                },
                SetCommands::RxEnable { value } => {
                    println!("Setting rx_enable to {}", value);
                    sx1255_info.rx_enable = *value;
                },
                SetCommands::RefEnable { value } => {
                    println!("Setting ref_enable to {}", value);
                    sx1255_info.ref_enable = *value;
                },
                SetCommands::RxFreq { freq } => {
                    println!("Setting Rx frequency to {}", *freq);
                    sx1255_info.rx_freq = *freq;
                },
                SetCommands::TxFreq { freq } => {
                    println!("Setting Tx frequency to {}", *freq);
                    sx1255_info.tx_freq = *freq;
                },
                SetCommands::TxDacGain { gain } => {
                    println!("Setting Tx DAC gain to {} ({})", *gain, OPTS.tx_dac_gain[*gain as usize]);
                    sx1255_info.tx_dac_gain = *gain;
                },
                SetCommands::TxMixerGain { gain } => {
                    println!("Setting Tx mixer gain to {} ({})", *gain, OPTS.tx_mixer_gain[*gain as usize]);
                    sx1255_info.tx_mixer_gain = *gain;
                },
                SetCommands::TxMixerTankCap { cap } => {
                    println!("Setting capacitance in parallel to mixer tank to {} ({})", *cap, OPTS.tx_mixer_tank_cap[*cap as usize]);
                    sx1255_info.tx_mixer_tank_cap = *cap;
                },
                SetCommands::TxPllBw { bw } => {
                    println!("Setting Tx PLL bandwidth to {} ({} KHz)", *bw, OPTS.tx_pll_bw[*bw as usize]);
                    sx1255_info.tx_pll_bw = *bw;
                },
                SetCommands::TxMixerTankRes { res } => {
                    println!("Setting resistance in parallel with the mixer tank to {} ({} kΩ)", *res, OPTS.tx_mixer_tank_res[*res as usize]);
                    sx1255_info.tx_pll_bw = *res;
                },
                SetCommands::TxFilterBw { bw } => {
                    println!("Setting Tx analog filter bandwidth DSB to {} ({} MHz)", *bw, OPTS.tx_filter_bw[*bw as usize]);
                    sx1255_info.tx_filter_bw = *bw;
                },
                SetCommands::TxDacBw { bw } => {
                    println!("Setting Tx FIR-DAC number of taps to {} ({} actual nubmer of taps)", *bw, OPTS.tx_dac_bw[*bw as usize]);
                    sx1255_info.tx_dac_bw = *bw;
                },
                SetCommands::RxLnaGain { gain } => {
                    println!("Setting Rx LNA gain to {} ({})", *gain, OPTS.rx_lna_gain[*gain as usize]);
                    sx1255_info.rx_lna_gain = *gain;
                },
                SetCommands::RxPgaGain { gain } => {
                    println!("Setting Rx PGA gain to {} ({})", *gain, OPTS.rx_pga_gain[*gain as usize]);
                    sx1255_info.rx_pga_gain = *gain;
                },
                SetCommands::RxZin200 { imp } => {
                    println!("Setting Rx input impedance to {} ({})", *imp, OPTS.rx_zin_200[*imp as usize]);
                    sx1255_info.rx_zin_200 = *imp;
                },
                SetCommands::RxAdcTrim { trim } => {
                    println!("Setting Rx ADC trim for 36 Mhz reference crystal to {}", *trim);
                    sx1255_info.rx_adc_trim = *trim;
                },
                SetCommands::RxPgaBw { bw } => {
                    println!("Setting Rx analog roofing filter to {} ({})", *bw, OPTS.rx_pga_bw[*bw as usize]);
                    sx1255_info.rx_pga_bw = *bw;
                },
                SetCommands::RxPllBw { bw } => {
                    println!("Setting Rx PLL bandwidth to {} ({})", *bw, OPTS.rx_pll_bw[*bw as usize]);
                    sx1255_info.rx_pll_bw = *bw;
                },
                SetCommands::RxAdcTemp { value } => {
                    println!("Setting Rx ADC temperature measurement mode to {}", *value);
                    sx1255_info.rx_adc_temp = *value;
                },
            };
            //set_info(&mut spi, sx1255_info);
        },
    }
}
