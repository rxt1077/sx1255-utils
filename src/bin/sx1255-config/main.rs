use std::{io};
use clap::{Parser, Subcommand};
use spidev::{Spidev, SpidevOptions, SpiModeFlags};
use std::path::PathBuf;
use gpio_cdev::{Chip, LineRequestFlags};

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
    /// Mapping of DIO(0)
    IoMap0 {
        /// 0=pll_lock_rx, 1=pll_lock_rx, 2=pll_lock_rx, 3=eol
        #[arg(value_parser=clap::value_parser!(u8).range(0..4))]
        value: u8,
    },
    /// Mapping of DIO(1)
    IoMap1 {
        /// 0=pll_lock_rx
        #[arg(value_parser=clap::value_parser!(u8).range(0..4))]
        value: u8,
    },
    /// Mapping of DIO(2)
    IoMap2 {
        /// 0=xosc_ready
        #[arg(value_parser=clap::value_parser!(u8).range(0..4))]
        value: u8,
    },
    /// Mapping of DIO(3)
    IoMap3 {
        /// 0=pll_lock rx in Rx mode & pll_lock_tx in all other modes
        #[arg(value_parser=clap::value_parser!(u8).range(0..4))]
        value: u8,
    },
    /// Enables the digital loop back mode of the frontend
    DigLoopbackEn {
        #[arg(action=clap::ArgAction::Set)]
        value: bool,
    },
    /// Enables the RF loop back mode of the frontend
    RfLoopbackEn {
        #[arg(action=clap::ArgAction::Set)]
        value: bool,
    },
    /// Disable IISM Rx (during Tx mode)
    IismRxDisable {
        #[arg(action=clap::ArgAction::Set)]
        value: bool,
    },
    /// Disable IISM Tx (during Rx mode)
    IismTxDisable {
        #[arg(action=clap::ArgAction::Set)]
        value: bool,
    },
    /// Sets the IISM mode
    IismMode {
        /// 0=A, 1=B1, 2=B2, 3=not used
        #[arg(value_parser=clap::value_parser!(u8).range(0..4))]
        mode: u8,
    },
    /// Sets the XTAL/CLK_OUT division factor
    IismClkDiv {
        /// 0=1, 1=2, 2=4, 3=8, 4=12, 5=16, 6=24, 7=32, 8=48, higher values not used
        #[arg(value_parser=clap::value_parser!(u8).range(0..16))]
        factor: u8,
    },
    /// Sets the interpolation/decimation factor
    R {
        /// 8, 16, 24, 32, 48, 64, 96, 128, 192, 256, 384, 512, 768, 1536
        /// 9, 18, 27, 36, 54, 72, 108, 144, 216, 288, 432, 576, 864, 1728
        #[arg(verbatim_doc_comment, value_parser=["8", "16", "24", "32", "48",
              "64", "96", "128", "192", "256", "384", "512", "768", "1536",
              "9", "18", "27", "36", "54", "72", "108", "144", "216", "288",
              "432", "576", "864", "1728"])]
        r_str: String,
    },
    /// Sets the IISM truncation mode
    IismTruncation {
        /// 0=MSB is truncated, alignment of LSB, 1=LSB is truncated, alignment on MSB
        #[arg(value_parser=clap::value_parser!(u8).range(0..2))]
        mode: u8,
    },
}

fn reset() -> Result<(), gpio_cdev::Error> {
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let output = chip.get_line(25)?;
    let output_handle = output.request(LineRequestFlags::OUTPUT, 0, "sx1255-config")?;
    output_handle.set_value(1)?;
    output_handle.set_value(0)?;
    Ok(())
}

fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open(SPI_DEV)?;
    spi.configure(&SPI_OPTS)?;
    Ok(spi)
}

fn main() {
    let mut spi = match create_spi() {
        Ok(spi) => spi,
        Err(e) => {
            println!("Unable to open SPI: {}", e);
            return
        },
    };

    let cli = Cli::parse();
    let mut sx1255_info = SX1255Info::default();
    get_info(&mut spi, &mut sx1255_info);

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
            set_info(&mut spi, sx1255_info);
        },
        Commands::Reset => {
            println!("Resetting");
            match reset() {
                Ok(_) => {},
                Err(e) => {
                    println!("Error during reset: {}", e);
                    println!("The pin may be in use by the deprecated sysfs interface.");
                    println!("Try running: echo 537 > /sys/class/gpio/unexport");
                    return
                },
            };
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
                SetCommands::IoMap0 { value } => {
                    println!("Setting mapping of DIO(0) {} ({})", *value, OPTS.iomap0[*value as usize]);
                    sx1255_info.iomap0 = *value;
                },
                SetCommands::IoMap1 { value } => {
                    println!("Setting mapping of DIO(1) {} ({})", *value, OPTS.iomap1[*value as usize]);
                    sx1255_info.iomap1 = *value;
                },
                SetCommands::IoMap2 { value } => {
                    println!("Setting mapping of DIO(2) {} ({})", *value, OPTS.iomap2[*value as usize]);
                    sx1255_info.iomap2= *value;
                },
                SetCommands::IoMap3 { value } => {
                    println!("Setting mapping of DIO(3) {} ({})", *value, OPTS.iomap3[*value as usize]);
                    sx1255_info.iomap3 = *value;
                },
                SetCommands::DigLoopbackEn { value } => {
                    println!("Setting digital loopback enable to {}", *value);
                    sx1255_info.dig_loopback_en = *value;
                },
                SetCommands::RfLoopbackEn { value } => {
                    println!("Setting digital loopback enable to {}", *value);
                    sx1255_info.rf_loopback_en = *value;
                },
                SetCommands::IismRxDisable { value } => {
                    println!("Setting flag to disable IISM Rx (during Tx mode) {}", *value);
                    sx1255_info.iism_rx_disable = *value;
                },
                SetCommands::IismTxDisable { value } => {
                    println!("Setting flag to disable IISM Tx (during Rx mode) {}", *value);
                    sx1255_info.iism_tx_disable = *value;
                },
                SetCommands::IismMode { mode } => {
                    println!("Setting IISM mode to {} ({})", *mode, OPTS.iism_mode[*mode as usize]);
                    sx1255_info.iism_mode = *mode;
                },
                SetCommands::IismClkDiv { factor } => {
                    println!("Setting XTAL/CLK_OUT division factor to {} ({})", *factor, OPTS.iism_clk_div[*factor as usize]);
                    sx1255_info.iism_clk_div = *factor;
                },
                SetCommands::R { r_str } => {
                    let r: u32 = r_str.parse().unwrap();
                    println!("Setting interpolation/decimation factor to {}", r);
                    sx1255_info.r = r;
                },
                SetCommands::IismTruncation { mode } => {
                    println!("Setting IISM truncation mode in Rx and Tx to {} ({})", *mode, OPTS.iism_truncation[*mode as usize]);
                    sx1255_info.iism_truncation = *mode;
                },
            };
            set_info(&mut spi, sx1255_info);
        },
    }
}
