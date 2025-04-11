use std::io;
use spidev::{Spidev, SpidevTransfer};

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

fn sx1255_readreg(spi: &mut Spidev, addr: u8) -> io::Result<u8> {
    let tx_buf = [addr, 0];
    let mut rx_buf = [0_u8; 2];
    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        spi.transfer(&mut transfer)?;
    }
    Ok(rx_buf[1])
}

fn sx1255_writereg(spi: &mut Spidev, addr: u8, val: u8) -> io::Result<u8> {
    let tx_buf = [addr | 0b10000000, val];
    let mut rx_buf = [0_u8; 2];
    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        spi.transfer(&mut transfer)?;
    }
    Ok(rx_buf[1])
}

pub struct SX1255Info {
    pub driver_enable: bool,
    pub tx_enable: bool,
    pub rx_enable: bool,
    pub ref_enable: bool,
    pub rx_freq: u32,
    pub tx_freq: u32,
    pub version: u8,
    pub tx_dac_gain: u8,
    pub tx_mixer_gain: u8,
    pub tx_mixer_tank_cap: u8,
    pub tx_mixer_tank_res: u8,
    pub tx_pll_bw: u8,
    pub tx_filter_bw: u8,
    pub tx_dac_bw: u8,
    pub rx_lna_gain: u8,
    pub rx_pga_gain: u8,
    pub rx_zin_200: u8,
    pub rx_adc_bw: u8,
    pub rx_adc_trim: u8,
    pub rx_pga_bw: u8,
    pub rx_pll_bw: u8,
    pub rx_adc_temp: bool,
    pub iomap0: u8,
    pub iomap1: u8,
    pub iomap2: u8,
    pub iomap3: u8,
    pub dig_loopback_en: bool,
    pub rf_loopback_en: bool,
    pub ckout_enable: u8,
    pub ck_select_tx_dac: u8,
    pub eol: u8,
    pub xosc_ready: bool,
    pub pll_lock_rx: bool,
    pub pll_lock_tx: bool,
    pub iism_rx_disable: bool,
    pub iism_tx_disable: bool,
    pub iism_mode: u8,
    pub iism_clk_div: u8,
    pub r: u32,
    pub iism_truncation: u8,
    pub iism_status_flag: u8,
}

// power on defaults for the device
impl Default for SX1255Info {
    fn default() -> SX1255Info {
        SX1255Info {
            driver_enable: false,
            tx_enable: false,
            rx_enable: false,
            ref_enable: true,
            rx_freq: 385777770,
            tx_freq: 385777770,
            version: 0x11,
            tx_dac_gain: 2,
            tx_mixer_gain: 12,
            tx_mixer_tank_cap: 3,
            tx_mixer_tank_res: 7,
            tx_pll_bw: 3,
            tx_filter_bw: 0,
            tx_dac_bw: 2,
            rx_lna_gain: 1,
            rx_pga_gain: 15,
            rx_zin_200: 1,
            rx_adc_bw: 7,
            rx_adc_trim: 7,
            rx_pga_bw: 1,
            rx_pll_bw: 3,
            rx_adc_temp: false,
            iomap0: 0, 
            iomap1: 0,
            iomap2: 0,
            iomap3: 0,
            dig_loopback_en: false,
            rf_loopback_en: false,
            ckout_enable: 1,
            ck_select_tx_dac: 0,
            eol: 0,
            xosc_ready: true,
            pll_lock_rx: false,
            pll_lock_tx: false,
            iism_rx_disable: false,
            iism_tx_disable: false,
            iism_mode: 0,
            iism_clk_div: 0,
            r: 8,
            iism_truncation: 0,
            iism_status_flag: 0,
        }

    }
}

static TX_DAC_GAIN_OPTS: [&str; 8]  = [
    "maximum gain - 9 dB",
    "maximum gain - 6 dB",
    "maximum gain - 3 dB",
    "maximum gain (0 dB full scale)",
    "Max gain - 9 dB with test Vref voltage",
    "Max gain - 6 dB with test Vref voltage",
    "Max gain - 3 dB with test Vref voltage",
    "Max gain, 0 dBFS with test Vref voltage",
];
static TX_MIXER_GAIN_OPTS: [&str; 16] = [
    "-37.5 dB", "-35.5 dB", "-33.5 dB", "-31.5 dB", "-29.5 dB", "-27.5 dB",
    "-25.5 dB", "-23.5 dB", "-21.5 dB", "-19.5 dB", "-17.5 dB", "-15.5 dB",
    "-13.5 dB", "-11.5 dB", "-9.5 dB", "-7.5 dB",
];
static TX_MIXER_TANK_CAP_OPTS: [&str; 8] = [
    "0 fF", "128 fF", "256 fF", "384 fF", "512 fF", "640 fF", "768 fF",
    "896 fF",
];
static TX_MIXER_TANK_RES_OPTS: [&str; 8] = [
    "0.95 kΩ", "1.11 kΩ", "1.32 kΩ", "1.65 kΩ", "2.18 kΩ", "3.24 kΩ",
    "6.00 kΩ", "none => about 64 kΩ",
];
static TX_PLL_BW_OPTS: [&str; 4] = ["75 KHz", "150 KHz", "225 KHz", "300 KHz",];
static TX_FILTER_BW_OPTS: [&str; 16] = [
    "0.418 Mhz", "0.429 Mhz", "0.440 Mhz", "0.451 Mhz", "0.464 Mhz",
    "0.476 Mhz", "0.490 Mhz", "0.504 Mhz", "0.520 Mhz", "0.546 Mhz",
    "0.553 Mhz", "0.572 Mhz", "0.591 Mhz", "0.613 Mhz", "0.635 Mhz",
    "0.660 Mhz",
];
static TX_DAC_BW_OPTS: [&str; 6] = [
    "24 taps", "32 taps", "40 taps", "48 taps", "56 taps", "64 taps",
];
static RX_LNA_GAIN_OPTS: [&str; 8] = [
    "not used", "highest gain power - 0 dB", "highest gain power - 6 dB",
    "highest gain power - 12 dB", "highest gain power - 24 dB",
    "highest gain power - 36 dB", "highest gain power - 48 dB", "not used",
];
static RX_PGA_GAIN_OPTS: [&str; 16] = [
    "lowest gain + 0 dB", "lowest gain + 2 dB", "lowest gain + 4 dB",
    "lowest gain + 6 dB", "lowest gain + 8 dB", "lowest gain + 10 db",
    "lowest gain + 12 dB", "lowest gain + 14 dB", "lowest gain + 16 dB",
    "lowest gain + 18 dB", "lowest gain + 20 dB", "lowest gain + 22 dB",
    "lowest gain + 24 dB", "lowest gain + 26 dB", "lowest gain + 28 dB",
    "lowest gain + 30 dB",
];
static RX_ZIN_200_OPTS: [&str; 2] = ["50Ω", "200Ω",];
static RX_ADC_BW_OPTS: [&str; 8] = [
    "unused", "use 0x01 instead ???", "100 kHz < BW < 400 kHz", "unused",
    "unused", "200 kHz < BW < 400 kHz", "unused", "BW > 400 kHz",
];
static RX_PGA_BW_OPTS: [&str; 4] = [
    "1500 kHz", "1000 kHz", "750 kHz", "500 kHz"
];
static RX_PLL_BW_OPTS: [&str; 4] = ["75 KHz", "150 KHz", "225 KHz", "300 KHz",];
static IOMAP0_OPTS: [&str; 4] = [
    "pll_lock_rx", "pll_lock_rx", "pll_lock_rx", "eol"
];
static IOMAP1_OPTS: [&str; 1] = ["pll_lock_tx"];
static IOMAP2_OPTS: [&str; 1] = ["xosc_ready"];
static IOMAP3_OPTS: [&str; 1] = ["pll_lock_rx in Rx mode & pll_lock_tx in all other modes"];
static CKOUT_ENABLE_OPTS: [&str; 2] = [
    "output clock disabled on pad CLK_OUT",
    "output clock enabled on pad CLK_OUT",
];
static CK_SELECT_TX_DAC_OPTS: [&str; 2] = [
    "internal clock (CLK_XTAL) used for Tx DAC",
    "external clock (CLK_IN) used for Tx DAC",
];
static EOL_OPTS: [&str; 2] = [
    "0 to VBAT > EOL threshold",
    "1 to VBAT < EOL threshold (battery low)",
];
static IISM_MODE_OPTS: [&str; 4] = ["mode A", "mode B1", "mode B2", "not used"];
static IISM_CLK_DIV_OPTS: [&str; 9] = [
    "1", "2", "4", "8", "12", "16", "24", "32", "48",
];
static IISM_TRUNCATION_OPTS: [&str; 2] = [
    "MSB is truncated, alignment on LSB",
    "LSB is truncated, alignment on MSB",
];
static IISM_STATUS_FLAG_OPTS: [&str; 2] = ["no error", "error, IISM off"];

pub fn print_info(sx1255_info: SX1255Info) {
    println!("
General Registers

               PA driver enabled: {driver_enable}
                      Tx enabled: {tx_enable}
                      Rx enabled: {rx_enable}
      PDS and oscillator enabled: {ref_enable}
            Rx carrier frequency: {rx_freq} Hz
            Tx carrier frequency: {tx_freq} Hz
                    Version code: 0x{version:02X}

Transmitter Front-End Configuration Registers

                     Tx DAC gain: {tx_dac_gain} ({tx_dac_gain_opt})
                   Tx mixer gain: {tx_mixer_gain} ({tx_mixer_gain_opt})
       Tx mixer tank capacitance: {tx_mixer_tank_cap} ({tx_mixer_tank_cap_opt})
        Tx mixer tank resistance: {tx_mixer_tank_res} ({tx_mixer_tank_res_opt})
                Tx PLL bandwidth: {tx_pll_bw} ({tx_pll_bw_opt})
             Tx filter bandwidth: {tx_filter_bw} ({tx_filter_bw_opt})
                Tx DAC bandwidth: {tx_dac_bw} ({tx_dac_bw_opt})

Receiver Front-End Configuration Registers

                     Rx LNA gain: {rx_lna_gain} ({rx_lna_gain_opts})
                     Rx PGA gain: {rx_pga_gain} ({rx_pga_gain_opts})
                 Input Impedance: {rx_zin_200} ({rx_zin_200_opts})
      Rx ΣΔ ADC BW configuration: {rx_adc_bw} ({rx_adc_bw_opts})
      Rx ADC trim 36 Hz ref xtal: {rx_adc_trim}
        Rx analog roofing filter: {rx_pga_bw} ({rx_pga_bw_opts})
                Rx PLL bandwidth: {rx_pll_bw} ({rx_pll_bw_opts})
        Rx ADC temp measure mode: {rx_adc_temp}

IRC and PIN Mapping Registers

               Mapping of DIO(0): {iomap0} ({iomap0_opt})
               Mapping of DIO(1): {iomap1} ({iomap1_opt})
               Mapping of DIO(2): {iomap2} ({iomap2_opt})
               Mapping of DIO(3): {iomap3} ({iomap3_opt})

Additional Parameter Configuration Registers

        Digital loopback enabled: {dig_loopback_en}
             RF loopback enabled: {rf_loopback_en}
               Clock out enabled: {ckout_enable} ({ckout_enable_opt})
         Clock select for Tx DAC: {ck_select_tx_dac} ({ck_select_tx_dac_opt})
               EOL output signal: {eol} ({eol_opt})
                   XOSC is ready: {xosc_ready}
                Rx PLL is locked: {pll_lock_rx}
                Tx PLL is locked: {pll_lock_tx}
Disable IISM Rx (during Tx mode): {iism_rx_disable}
Disable IISM Tx (during Rx mode): {iism_tx_disable}
                       IISM Mode: {iism_mode} ({iism_mode_opt})
    XTAL/CLK_OUT division factor: {iism_clk_div} ({iism_clk_div_opt})
 Decimation/Interpolation factor: {r}
            IISM truncation mode: {iism_truncation} ({iism_truncation_opt})
          IISM error status flag: {iism_status_flag} ({iism_status_flag_opt})
",
        driver_enable         = sx1255_info.driver_enable,
        tx_enable             = sx1255_info.tx_enable,
        rx_enable             = sx1255_info.rx_enable,
        ref_enable            = sx1255_info.ref_enable,
        rx_freq               = sx1255_info.rx_freq,
        tx_freq               = sx1255_info.tx_freq,
        version               = sx1255_info.version,
        tx_dac_gain           = sx1255_info.tx_dac_gain,
        tx_dac_gain_opt       = TX_DAC_GAIN_OPTS[sx1255_info.tx_dac_gain as usize],
        tx_mixer_gain         = sx1255_info.tx_mixer_gain,
        tx_mixer_gain_opt     = TX_MIXER_GAIN_OPTS[sx1255_info.tx_mixer_gain as usize],
        tx_mixer_tank_cap     = sx1255_info.tx_mixer_tank_cap,
        tx_mixer_tank_cap_opt = TX_MIXER_TANK_CAP_OPTS[sx1255_info.tx_mixer_tank_cap as usize],
        tx_mixer_tank_res     = sx1255_info.tx_mixer_tank_res,
        tx_mixer_tank_res_opt = TX_MIXER_TANK_RES_OPTS[sx1255_info.tx_mixer_tank_res as usize],
        tx_pll_bw             = sx1255_info.tx_pll_bw,
        tx_pll_bw_opt         = TX_PLL_BW_OPTS[sx1255_info.tx_pll_bw as usize],
        tx_filter_bw          = sx1255_info.tx_filter_bw,
        tx_filter_bw_opt=TX_FILTER_BW_OPTS[sx1255_info.tx_filter_bw as usize],
        tx_dac_bw=sx1255_info.tx_dac_bw,
        tx_dac_bw_opt=TX_DAC_BW_OPTS[sx1255_info.tx_dac_bw as usize],
        rx_lna_gain=sx1255_info.rx_lna_gain,
        rx_lna_gain_opts=RX_LNA_GAIN_OPTS[sx1255_info.rx_lna_gain as usize],
        rx_pga_gain=sx1255_info.rx_pga_gain,
        rx_pga_gain_opts=RX_PGA_GAIN_OPTS[sx1255_info.rx_pga_gain as usize],
        rx_zin_200=sx1255_info.rx_zin_200,
        rx_zin_200_opts=RX_ZIN_200_OPTS[sx1255_info.rx_zin_200 as usize],
        rx_adc_bw=sx1255_info.rx_adc_bw,
        rx_adc_bw_opts=RX_ADC_BW_OPTS[sx1255_info.rx_adc_bw as usize],
        rx_adc_trim=sx1255_info.rx_adc_trim,
        rx_pga_bw=sx1255_info.rx_pga_bw,
        rx_pga_bw_opts=RX_PGA_BW_OPTS[sx1255_info.rx_pga_bw as usize],
        rx_pll_bw=sx1255_info.rx_pll_bw,
        rx_pll_bw_opts=RX_PLL_BW_OPTS[sx1255_info.rx_pll_bw as usize],
        rx_adc_temp=sx1255_info.rx_adc_temp,
        iomap0=sx1255_info.iomap0,
        iomap0_opt=IOMAP0_OPTS[sx1255_info.iomap0 as usize],
        iomap1=sx1255_info.iomap1,
        iomap1_opt=IOMAP1_OPTS[sx1255_info.iomap1 as usize],
        iomap2=sx1255_info.iomap2,
        iomap2_opt=IOMAP2_OPTS[sx1255_info.iomap2 as usize],
        iomap3=sx1255_info.iomap3,
        iomap3_opt=IOMAP3_OPTS[sx1255_info.iomap3 as usize],
        dig_loopback_en=sx1255_info.dig_loopback_en,
        rf_loopback_en=sx1255_info.rf_loopback_en,
        ckout_enable=sx1255_info.ckout_enable,
        ckout_enable_opt=CKOUT_ENABLE_OPTS[sx1255_info.ckout_enable as usize],
        ck_select_tx_dac=sx1255_info.ck_select_tx_dac,
        ck_select_tx_dac_opt=CK_SELECT_TX_DAC_OPTS[sx1255_info.ck_select_tx_dac as usize],
        eol=sx1255_info.eol,
        eol_opt=EOL_OPTS[sx1255_info.eol as usize],
        xosc_ready=sx1255_info.xosc_ready,
        pll_lock_rx=sx1255_info.pll_lock_rx,
        pll_lock_tx=sx1255_info.pll_lock_tx,
        iism_rx_disable=sx1255_info.iism_rx_disable,
        iism_tx_disable=sx1255_info.iism_tx_disable,
        iism_mode=sx1255_info.iism_mode,
        iism_mode_opt=IISM_MODE_OPTS[sx1255_info.iism_mode as usize],
        iism_clk_div=sx1255_info.iism_clk_div,
        iism_clk_div_opt=IISM_CLK_DIV_OPTS[sx1255_info.iism_clk_div as usize],
        r = sx1255_info.r,
        iism_truncation = sx1255_info.iism_truncation,
        iism_truncation_opt = IISM_TRUNCATION_OPTS[sx1255_info.iism_truncation as usize],
        iism_status_flag = sx1255_info.iism_status_flag,
        iism_status_flag_opt = IISM_STATUS_FLAG_OPTS[sx1255_info.iism_status_flag as usize],
    );
}

fn freq_to_u32(frfh: u8, frfm: u8, frfl: u8) -> u32 {
    (((((frfh as u32) << 16) +
       ((frfm as u32) << 8) +
       ((frfl as u32) << 0)) as f64)
     * (32000000.0 / 1048576.0)) as u32
}

fn calc_r(mant: u8, m: u8, n: u8) -> u32 {
    // r = MANT*3^m*2^n
    // where MANT is 8 for the 1st set and 9 for the second set,
    // m can be 0 or 1, and n is an integer between 0 and 6
    
    (mant as u32) * 3_u32.pow(m.into()) * 2_u32.pow(n.into())
}

pub fn get_info(spi: &mut Spidev, sx1255_info: &mut SX1255Info) {

            // read the registers
            let mode       = sx1255_readreg(spi, REG_MODE).expect("read mode register");
            let frfh_rx    = sx1255_readreg(spi, REG_FRFH_RX).expect("read FRFH_RX register");
            let frfm_rx    = sx1255_readreg(spi, REG_FRFM_RX).expect("read FRFM_RX register");
            let frfl_rx    = sx1255_readreg(spi, REG_FRFL_RX).expect("read FRFL_RX register");
            let frfh_tx    = sx1255_readreg(spi, REG_FRFH_TX).expect("read FRFH_TX register");
            let frfm_tx    = sx1255_readreg(spi, REG_FRFM_TX).expect("read FRFM_TX register");
            let frfl_tx    = sx1255_readreg(spi, REG_FRFL_TX).expect("read FRFL_TX register");
            let version    = sx1255_readreg(spi, REG_VERSION).expect("read version register");
            let txfe1      = sx1255_readreg(spi, REG_TXFE1).expect("read TXFE1 register");
            let txfe2      = sx1255_readreg(spi, REG_TXFE2).expect("read TXFE2 register");
            let txfe3      = sx1255_readreg(spi, REG_TXFE3).expect("read TXFE3 register");
            let txfe4      = sx1255_readreg(spi, REG_TXFE4).expect("read TXFE4 register");
            let rxfe1      = sx1255_readreg(spi, REG_RXFE1).expect("read RXFE1 register");
            let rxfe2      = sx1255_readreg(spi, REG_RXFE2).expect("read RXFE2 register");
            let rxfe3      = sx1255_readreg(spi, REG_RXFE3).expect("read RXFE3 register");
            let iomap      = sx1255_readreg(spi, REG_IO_MAP).expect("read IO_MAP register");
            let ck_sel     = sx1255_readreg(spi, REG_CK_SEL).expect("read CK_SEL register");
            let stat       = sx1255_readreg(spi, REG_STAT).expect("read STAT register");
            let iism       = sx1255_readreg(spi, REG_IISM).expect("read IISM register");
            let dig_bridge = sx1255_readreg(spi, REG_DIG_BRIDGE).expect("read DIG_BRIDGE register");

            // calculate the decimation/interpolation factor
            let r = calc_r(
                if ((dig_bridge & 0b10000000) >> 7) == 0 { 8 } else { 9 }, // MANT is 8 or 9 depending on bit
                (dig_bridge & 0b01000000) >> 6, // m
                (dig_bridge & 0b00111000) >> 3, // n
            );

            // put the values in the struct
            sx1255_info.driver_enable     = (mode & 0b00001000) != 0;
            sx1255_info.tx_enable         = (mode & 0b00000100) != 0;
            sx1255_info.rx_enable         = (mode & 0b00000010) != 0;
            sx1255_info.ref_enable        = (mode & 0b00000001) != 0;
            sx1255_info.rx_freq           = freq_to_u32(frfh_rx, frfm_rx, frfl_rx);
            sx1255_info.tx_freq           = freq_to_u32(frfh_tx, frfm_tx, frfl_tx);
            sx1255_info.version           = version;
            sx1255_info.tx_dac_gain       = (txfe1 & 0b01110000) >> 4;
            sx1255_info.tx_mixer_gain     =  txfe1 & 0b00001111;
            sx1255_info.tx_mixer_tank_cap = (txfe2 & 0b00111000) >> 3;
            sx1255_info.tx_mixer_tank_res =  txfe2 & 0b00000111;
            sx1255_info.tx_pll_bw         = (txfe3 & 0b01100000) >> 5;
            sx1255_info.tx_filter_bw      =  txfe3 & 0b00001111;
            sx1255_info.tx_dac_bw         =  txfe4 & 0b00000111;
            sx1255_info.rx_lna_gain       = (rxfe1 & 0b11100000) >> 5;
            sx1255_info.rx_pga_gain       = (rxfe1 & 0b00011110) >> 1;
            sx1255_info.rx_zin_200        =  rxfe1 & 0b00000001;
            sx1255_info.rx_adc_bw         = (rxfe2 & 0b11100000) >> 5;
            sx1255_info.rx_adc_trim       = (rxfe2 & 0b00011100) >> 2;
            sx1255_info.rx_pga_bw         =  rxfe2 & 0b00000011;
            sx1255_info.rx_pll_bw         = (rxfe3 & 0b00000110) >> 1;
            sx1255_info.rx_adc_temp       = (rxfe3 & 0b00000001) != 0;
            sx1255_info.iomap0            = (iomap & 0b11000000) >> 6;
            sx1255_info.iomap1            = (iomap & 0b00110000) >> 4;
            sx1255_info.iomap2            = (iomap & 0b00001100) >> 2;
            sx1255_info.iomap3            =  iomap & 0b00000011;
            sx1255_info.dig_loopback_en   = ((ck_sel & 0b00001000) >> 3) != 0;
            sx1255_info.rf_loopback_en    = ((ck_sel & 0b00000100) >> 2) != 0;
            sx1255_info.ckout_enable      =  (ck_sel & 0b00000010) >> 1;
            sx1255_info.ck_select_tx_dac  =   ck_sel & 0b00000001;
            sx1255_info.eol               =  (stat & 0b00001000) >> 3;
            sx1255_info.xosc_ready        = ((stat & 0b00000100) >> 2) != 0;
            sx1255_info.pll_lock_rx       = ((stat & 0b00000010) >> 1) != 0;
            sx1255_info.pll_lock_tx       =  (stat & 0b00000001) != 0;
            sx1255_info.iism_rx_disable   = ((iism & 0b10000000) >> 7) != 0;
            sx1255_info.iism_tx_disable   = ((iism & 0b01000000) >> 6) != 0;
            sx1255_info.iism_mode         =  (iism & 0b00110000) >> 4;
            sx1255_info.iism_clk_div      =   iism & 0b00001111;
            sx1255_info.iism_truncation   = (dig_bridge & 0b00000100) >> 2;
            sx1255_info.iism_status_flag  = (dig_bridge & 0b00000010) >> 1;
            sx1255_info.r                 = r;
}

fn bool_to_u8(flag: bool) -> u8 {
    if flag { 1 } else { 0 }
}

// returns high, middle, low
fn u32_to_freq(freq: u32) -> (u8, u8, u8) {
    let adjusted_freq = ((freq as f64) * (1048576.0 / 32000000.0)) as u32;
    ((adjusted_freq >> 16) as u8, (adjusted_freq >> 8) as u8, adjusted_freq as u8)
}

pub static VALID_R_VALUES: [u32; 28]  = [
    8, 16, 24, 32, 48, 64, 96, 128, 192, 256, 384, 512, 768, 1536,  // set 1
    9, 18, 27, 36, 54, 72, 108, 144, 216, 288, 432, 576, 864, 1728, // set 2
];

fn r_to_mant_m_n(r: u32) -> (u8, u8, u8) {
// the easiest way I could think to do this was with a simple match
    match r {
        // set 1
        8    => (8,0,0),
        16   => (8,0,1),
        24   => (8,1,0),
        32   => (8,0,2),
        48   => (8,1,1),
        64   => (8,0,3),
        96   => (8,1,2),
        128  => (8,0,4),
        192  => (8,1,3),
        256  => (8,0,5),
        384  => (8,1,4),
        512  => (8,0,6),
        768  => (8,1,5),
        1536 => (8,1,6),
        // set 2
        9    => (9,0,0),
        18   => (9,0,1),
        27   => (9,1,0),
        36   => (9,0,2),
        54   => (9,1,1),
        72   => (9,0,3),
        108  => (9,1,2),
        144  => (9,0,4),
        216  => (9,1,3),
        288  => (9,0,5),
        432  => (9,1,4),
        576  => (9,0,6),
        864  => (9,1,5),
        1728 => (9,1,6),
        _    => panic!("invalid r value"),
    }
}

pub fn set_info(spi: &mut Spidev, sx1255_info: SX1255Info) {

    // build the register values from the info struct
    let mode = bool_to_u8(sx1255_info.driver_enable) << 3 |
               bool_to_u8(sx1255_info.tx_enable)     << 2 |
               bool_to_u8(sx1255_info.rx_enable)     << 1 |
               bool_to_u8(sx1255_info.ref_enable);
    let (frfh_rx, frfm_rx, frfl_rx) = u32_to_freq(sx1255_info.rx_freq);
    let (frfh_tx, frfm_tx, frfl_tx) = u32_to_freq(sx1255_info.tx_freq);
    let txfe1 = sx1255_info.tx_dac_gain << 4 |
                sx1255_info.tx_mixer_gain;
    let txfe2 = sx1255_info.tx_mixer_tank_cap << 3 |
                sx1255_info.tx_mixer_tank_res;
    let txfe3 = sx1255_info.tx_pll_bw << 5 |
                sx1255_info.tx_filter_bw;
    let txfe4 = sx1255_info.tx_dac_bw;
    let rxfe1 = sx1255_info.rx_lna_gain << 5 |
                sx1255_info.rx_pga_gain << 1 |
                sx1255_info.rx_zin_200;
    let rxfe2 = sx1255_info.rx_adc_bw   << 5 |
                sx1255_info.rx_adc_trim << 2 |
                sx1255_info.rx_pga_bw;
    let rxfe3 = sx1255_info.rx_pll_bw << 1 |
                bool_to_u8(sx1255_info.rx_adc_temp);
    let iomap = sx1255_info.iomap0 << 6 |
                sx1255_info.iomap1 << 4 |
                sx1255_info.iomap2 << 2 |
                sx1255_info.iomap3;
    let ck_sel = bool_to_u8(sx1255_info.dig_loopback_en) << 3 |
                 bool_to_u8(sx1255_info.rf_loopback_en)  << 2 |
                 sx1255_info.ckout_enable                << 1 |
                 sx1255_info.ck_select_tx_dac;
    let iism = bool_to_u8(sx1255_info.iism_rx_disable) << 7 |
               bool_to_u8(sx1255_info.iism_tx_disable) << 6 |
               sx1255_info.iism_mode                   << 4 |
               sx1255_info.iism_clk_div;
    let (mant, m, n) = r_to_mant_m_n(sx1255_info.r);
    let dig_bridge = mant                        << 7 |
                     m                           << 6 |
                     n                           << 3 |
                     sx1255_info.iism_truncation << 2;

    // write the registers
    _ = sx1255_writereg(spi, REG_MODE, mode);
    _ = sx1255_writereg(spi, REG_FRFH_RX, frfh_rx);
    _ = sx1255_writereg(spi, REG_FRFM_RX, frfm_rx);
    _ = sx1255_writereg(spi, REG_FRFL_RX, frfl_rx);
    _ = sx1255_writereg(spi, REG_FRFH_TX, frfh_tx);
    _ = sx1255_writereg(spi, REG_FRFM_TX, frfm_tx);
    _ = sx1255_writereg(spi, REG_FRFL_TX, frfl_tx);
    _ = sx1255_writereg(spi, REG_TXFE1, txfe1);
    _ = sx1255_writereg(spi, REG_TXFE2, txfe2);
    _ = sx1255_writereg(spi, REG_TXFE3, txfe3);
    _ = sx1255_writereg(spi, REG_TXFE4, txfe4);
    _ = sx1255_writereg(spi, REG_RXFE1, rxfe1);
    _ = sx1255_writereg(spi, REG_RXFE2, rxfe2);
    _ = sx1255_writereg(spi, REG_RXFE3, rxfe3);
    _ = sx1255_writereg(spi, REG_IO_MAP, iomap);
    _ = sx1255_writereg(spi, REG_CK_SEL, ck_sel);
    _ = sx1255_writereg(spi, REG_IISM, iism);
    _ = sx1255_writereg(spi, REG_DIG_BRIDGE, dig_bridge);
}
