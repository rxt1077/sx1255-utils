use std::io;
use clap::{Parser, Subcommand};
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SpiModeFlags};
use std::path::PathBuf;

static SPI_DEV: &str = "/dev/spidev0.0";
static SPI_OPTS: SpidevOptions = SpidevOptions {
    bits_per_word: Some(8),
    max_speed_hz: Some(500000),
    lsb_first: Some(false),
    spi_mode: Some(SpiModeFlags::SPI_MODE_0),
};

static REG_MODE: u8       = 0x00;
static REG_FRFH_RX: u8    = 0x01;
static REG_FRFM_RX: u8    = 0x02;
static REG_FRFL_RX: u8    = 0x03;
static REG_FRFH_TX: u8    = 0x04;
static REG_FRFM_TX: u8    = 0x05;
static REG_FRFL_TX: u8    = 0x06;
static REG_VERSION: u8    = 0x07;
static REG_TXFE1: u8      = 0x08;
static REG_TXFE2: u8      = 0x09;
static REG_TXFE3: u8      = 0x0A;
static REG_TXFE4: u8      = 0x0B;
static REG_RXFE1: u8      = 0x0C;
static REG_RXFE2: u8      = 0x0D;
static REG_RXFE3: u8      = 0x0E;
static REG_IO_MAP: u8     = 0x0F;
static REG_CK_SEL: u8     = 0x10;
static REG_STAT: u8       = 0x11;
static REG_IISM: u8       = 0x12;
static REG_DIG_BRIDGE: u8 = 0x13;

static TX_DAC_GAIN_DESC: [&str; 8]  = [
    "maximum gain - 9 dB",
    "maximum gain - 6 dB",
    "maximum gain - 3 dB",
    "maximum gain (0 dB full scale)",
    "Max gain - 9 dB with test Vref voltage",
    "Max gain - 6 dB with test Vref voltage",
    "Max gain - 3 dB with test Vref voltage",
    "Max gain, 0 dBFS with test Vref voltage",
];
static TX_MIXER_GAIN_DESC: [&str; 16] = [
    "-37.5 dB", "-35.5 dB", "-33.5 dB", "-31.5 dB", "-29.5 dB", "-27.5 dB",
    "-25.5 dB", "-23.5 dB", "-21.5 dB", "-19.5 dB", "-17.5 dB", "-15.5 dB",
    "-13.5 dB", "-11.5 dB", "-9.5 dB", "-7.5 dB",
];
static TX_MIXER_TANK_CAP_DESC: [&str; 8] = [
    "0 fF", "128 fF", "256 fF", "384 fF", "512 fF", "640 fF", "768 fF",
    "896 fF",
];
static TX_MIXER_TANK_RES_DESC: [&str; 8] = [
    "0.95 kΩ", "1.11 kΩ", "1.32 kΩ", "1.65 kΩ", "2.18 kΩ", "3.24 kΩ",
    "6.00 kΩ", "none => about 64 kΩ",
];
static TX_PLL_BW_DESC: [&str; 4] = ["75 KHz", "150 KHz", "225 KHz", "300 KHz",];
static TX_FILTER_BW_DESC: [&str; 16] = [
    "0.418 Mhz", "0.429 Mhz", "0.440 Mhz", "0.451 Mhz", "0.464 Mhz",
    "0.476 Mhz", "0.490 Mhz", "0.504 Mhz", "0.520 Mhz", "0.546 Mhz",
    "0.553 Mhz", "0.572 Mhz", "0.591 Mhz", "0.613 Mhz", "0.635 Mhz",
    "0.660 Mhz",
];
static TX_DAC_BW_DESC: [&str; 6] = [
    "24 taps", "32 taps", "40 taps", "48 taps", "56 taps", "64 taps",
];
static RX_LNA_GAIN_DESC: [&str; 8] = [
    "not used", "highest gain power - 0 dB", "highest gain power - 6 dB",
    "highest gain power - 12 dB", "highest gain power - 24 dB",
    "highest gain power - 36 dB", "highest gain power - 48 dB", "not used",
];
static RX_PGA_GAIN_DESC: [&str; 16] = [
    "lowest gain + 0 dB", "lowest gain + 2 dB", "lowest gain + 4 dB",
    "lowest gain + 6 dB", "lowest gain + 8 dB", "lowest gain + 10 db",
    "lowest gain + 12 dB", "lowest gain + 14 dB", "lowest gain + 16 dB",
    "lowest gain + 18 dB", "lowest gain + 20 dB", "lowest gain + 22 dB",
    "lowest gain + 24 dB", "lowest gain + 26 dB", "lowest gain + 28 dB",
    "lowest gain + 30 dB",
];
static RX_ZIN_200_DESC: [&str; 2] = ["50Ω", "200Ω",];
static RX_ADC_BW_DESC: [&str; 8] = [
    "unused", "use 0x01 instead ???", "100 kHz < BW < 400 kHz", "unused",
    "unused", "200 kHz < BW < 400 kHz", "unused", "BW > 400 kHz",
];
static RX_PGA_BW_DESC: [&str; 4] = [
    "1500 kHz", "1000 kHz", "750 kHz", "500 kHz"
];
static RX_PLL_BW_DESC: [&str; 4] = ["75 KHz", "150 KHz", "225 KHz", "300 KHz",];
static IOMAP0_DESC: [&str; 4] = [
    "pll_lock_rx", "pll_lock_rx", "pll_lock_rx", "eol"
];
static IOMAP1_DESC: [&str; 1] = ["pll_lock_tx"];
static IOMAP2_DESC: [&str; 1] = ["xosc_ready"];
static IOMAP3_DESC: [&str; 1] = ["pll_lock_rx in Rx mode & pll_lock_tx in all other modes"];
static CKOUT_ENABLE_DESC: [&str; 2] = [
    "internal clock (CLK_XTAL) used for Tx DAC",
    "external clock (CLK_IN) used to Tx DAC"
];
static CK_SELECT_TX_DAC_DESC: [&str; 2] = [
    "internal clock (CLK_XTAL) used for Tx DAC",
    "external clock (CLK_IN) used for Tx DAC",
];
static EOL_DESC: [&str; 2] = [
    "0 to VBAT > EOL threshold",
    "1 to VBAT < EOL threshold (battery low)",
];
static IISM_MODE_DESC: [&str; 4] = ["mode A", "mode B1", "mode B2", "not used"];
static IISM_CLK_DIV_DESC: [&str; 9] = [
    "1", "2", "4", "8", "12", "16", "24", "32", "48",
];

struct SX1255Info {
    driver_enable: bool,
    tx_enable: bool,
    rx_enable: bool,
    ref_enable: bool,
    rx_freq: u32,
    tx_freq: u32,
    version: u8,
    tx_dac_gain: u8,
    tx_mixer_gain: u8,
    tx_mixer_tank_cap: u8,
    tx_mixer_tank_res: u8,
    tx_pll_bw: u8,
    tx_filter_bw: u8,
    tx_dac_bw: u8,
    rx_lna_gain: u8,
    rx_pga_gain: u8,
    rx_zin_200: u8,
    rx_adc_bw: u8,
    rx_adc_trim: u8,
    rx_pga_bw: u8,
    rx_pll_bw: u8,
    rx_adc_temp: bool,
    iomap0: u8,
    iomap1: u8,
    iomap2: u8,
    iomap3: u8,
    dig_loopback_en: bool,
    rf_loopback_en: bool,
    ckout_enable: u8,
    ck_select_tx_dac: u8,
    eol: u8,
    xosc_ready: bool,
    pll_lock_rx: bool,
    pll_lock_tx: bool,
    iism_rx_disable: bool,
    iism_tx_disable: bool,
    iism_mode: u8,
    iism_clk_div: u8,
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
enum SetCommands {
    /// Enables the PA driver
    DriverEnable {
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
            let rxfe1 = sx1255_readreg(&mut spi, REG_RXFE1).expect("read RXFE1 register");
            let rxfe2 = sx1255_readreg(&mut spi, REG_RXFE2).expect("read RXFE2 register");
            let rxfe3 = sx1255_readreg(&mut spi, REG_RXFE3).expect("read RXFE3 register");
            let iomap = sx1255_readreg(&mut spi, REG_IO_MAP).expect("read IO_MAP register");
            let ck_sel = sx1255_readreg(&mut spi, REG_CK_SEL).expect("read CK_SEL register");
            let stat = sx1255_readreg(&mut spi, REG_STAT).expect("read STAT register");
            let iism = sx1255_readreg(&mut spi, REG_IISM).expect("read IISM register");
            let dig_bridge = sx1255_readreg(&mut spi, REG_DIG_BRIDGE).expect("read DIG_BRIDGE register");

            let info = SX1255Info {
                driver_enable: (mode & 0b00001000) != 0,
                tx_enable:     (mode & 0b00000100) != 0,
                rx_enable:     (mode & 0b00000010) != 0,
                ref_enable:    (mode & 0b00000001) != 0,
                rx_freq: freq_to_u32(frfh_rx, frfm_rx, frfl_rx),
                tx_freq: freq_to_u32(frfh_tx, frfm_tx, frfl_tx),
                version: version,
                tx_dac_gain: (txfe1 & 0b01110000) >> 4,
                tx_mixer_gain: txfe1 & 0b00001111,
                tx_mixer_tank_cap: (txfe2 & 0b00111000) >> 3,
                tx_mixer_tank_res: txfe2 & 0b00000111,
                tx_pll_bw: (txfe3 & 0b01100000) >> 5,
                tx_filter_bw: txfe3 & 0b00001111,
                tx_dac_bw: txfe4 & 0b00000111,
                rx_lna_gain: (rxfe1 & 0b11100000) >> 5,
                rx_pga_gain: (rxfe1 & 0b00011110) >> 1,
                rx_zin_200: rxfe1 & 0b00000001,
                rx_adc_bw: (rxfe2 & 0b11100000) >> 5,
                rx_adc_trim: (rxfe2 & 0b00011100) >> 2,
                rx_pga_bw: rxfe2 & 0b00000011,
                rx_pll_bw: (rxfe3 & 0b00000110) >> 1,
                rx_adc_temp: (rxfe3 & 0b00000001) != 0,
                iomap0: (iomap & 0b11000000) >> 6,
                iomap1: (iomap & 0b00110000) >> 4,
                iomap2: (iomap & 0b00001100) >> 2,
                iomap3: iomap & 0b00000011,
                dig_loopback_en: ((ck_sel & 0b00001000) >> 3) != 0,
                rf_loopback_en:  ((ck_sel & 0b00000100) >> 2) != 0,
                ckout_enable:     (ck_sel & 0b00000010) >> 1,
                ck_select_tx_dac:  ck_sel & 0b00000001,
                eol:          (stat & 0b00001000) >> 3,
                xosc_ready:  ((stat & 0b00000100) >> 2) != 0,
                pll_lock_rx: ((stat & 0b00000010) >> 1) != 0,
                pll_lock_tx:  (stat & 0b00000001) != 0,
                iism_rx_disable: ((iism & 0b10000000) >> 7) != 0,
                iism_tx_disable: ((iism & 0b01000000) >> 6) != 0,
                iism_mode:        (iism & 0b00110000) >> 4,
                iism_clk_div:      iism & 0b00001111,
            };

            println!("General Registers");
            println!("");
            println!("               PA driver enabled: {}", info.driver_enable);
            println!("                      Tx enabled: {}", info.tx_enable);
            println!("                      Rx enabled: {}", info.rx_enable);
            println!("      PDS and oscillator enabled: {}", info.ref_enable);
            println!("            Rx carrier frequency: {} Hz", info.rx_freq);
            println!("            Tx carrier frequency: {} Hz", info.tx_freq);
            println!("                    Version code: 0x{:02X}", info.version);
            println!("");
            println!("Transmitter Front-End Configuration Registers");
            println!("");
            println!("                     Tx DAC gain: {} ({})", info.tx_dac_gain, TX_DAC_GAIN_DESC[info.tx_dac_gain as usize]);
            println!("                   Tx mixer gain: {} ({})", info.tx_mixer_gain, TX_MIXER_GAIN_DESC[info.tx_mixer_gain as usize]);
            println!("       Tx mixer tank capacitance: {} ({})", info.tx_mixer_tank_cap, TX_MIXER_TANK_CAP_DESC[info.tx_mixer_tank_cap as usize]);
            println!("        Tx mixer tank resistance: {} ({})", info.tx_mixer_tank_res, TX_MIXER_TANK_RES_DESC[info.tx_mixer_tank_res as usize]);
            println!("                Tx PLL bandwidth: {} ({})", info.tx_pll_bw, TX_PLL_BW_DESC[info.tx_pll_bw as usize]);
            println!("             Tx filter bandwidth: {} ({})", info.tx_filter_bw, TX_FILTER_BW_DESC[info.tx_filter_bw as usize]);
            println!("                Tx DAC bandwidth: {} ({})", info.tx_dac_bw, TX_DAC_BW_DESC[info.tx_dac_bw as usize]);
            println!("");
            println!("Receiver Front-End Configuration Registers");
            println!("");
            println!("                     Rx LNA gain: {} ({})", info.rx_lna_gain, RX_LNA_GAIN_DESC[info.rx_lna_gain as usize]);
            println!("                     Rx PGA gain: {} ({})", info.rx_pga_gain, RX_PGA_GAIN_DESC[info.rx_pga_gain as usize]);
            println!("                 Input Impedance: {} ({})", info.rx_zin_200, RX_ZIN_200_DESC[info.rx_zin_200 as usize]);
            println!("      Rx ΣΔ ADC BW configuration: {} ({})", info.rx_adc_bw, RX_ADC_BW_DESC[info.rx_adc_bw as usize]);
            println!("      Rx ADC trim 36 Hz ref xtal: {}", info.rx_adc_trim);
            println!("        Rx analog roofing filter: {} ({})", info.rx_pga_bw, RX_PGA_BW_DESC[info.rx_pga_bw as usize]);
            println!("                Rx PLL bandwidth: {} ({})", info.rx_pll_bw, RX_PLL_BW_DESC[info.rx_pll_bw as usize]);
            println!("        Rx ADC temp measure mode: {}", info.rx_adc_temp);
            println!("");
            println!("IRC and PIN Mapping Registers");
            println!("");
            println!("               Mapping of DIO(0): {} ({})", info.iomap0, IOMAP0_DESC[info.iomap0 as usize]);
            println!("               Mapping of DIO(1): {} ({})", info.iomap1, IOMAP1_DESC[info.iomap1 as usize]);
            println!("               Mapping of DIO(2): {} ({})", info.iomap2, IOMAP2_DESC[info.iomap2 as usize]);
            println!("               Mapping of DIO(3): {} ({})", info.iomap3, IOMAP3_DESC[info.iomap3 as usize]);
            println!("");
            println!("Additional Parameter Configuration Registers");
            println!("");
            println!("        Digital loopback enabled: {}", info.dig_loopback_en);
            println!("             RF loopback enabled: {}", info.rf_loopback_en);
            println!("               Clock out enabled: {} ({})", info.ckout_enable, CKOUT_ENABLE_DESC[info.ckout_enable as usize]);
            println!("         Clock select for Tx DAC: {} ({})", info.ck_select_tx_dac, CK_SELECT_TX_DAC_DESC[info.ck_select_tx_dac as usize]);
            println!("               EOL output signal: {} ({})", info.eol, EOL_DESC[info.eol as usize]);
            println!("                   XOSC is ready: {}", info.xosc_ready);
            println!("                Rx PLL is locked: {}", info.pll_lock_rx);
            println!("                Tx PLL is locked: {}", info.pll_lock_tx);
            println!("Disable IISM Rx (during Tx mode): {}", info.iism_rx_disable);
            println!("Disable IISM Tx (during Rx mode): {}", info.iism_tx_disable);
            println!("                       IISM Mode: {} ({})", info.iism_mode, IISM_MODE_DESC[info.iism_mode as usize]);
            println!("    XTAL/CLK_OUT division factor: {} ({})", info.iism_clk_div, IISM_CLK_DIV_DESC[info.iism_clk_div as usize]);
            println!("")
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
            println!("Set");
        },
    }
}
