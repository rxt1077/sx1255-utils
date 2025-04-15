pub struct SettingOptions<'a> {
    pub tx_dac_gain:       [&'a str; 8],
    pub tx_mixer_gain:     [&'a str; 16],
    pub tx_mixer_tank_cap: [&'a str; 8],
    pub tx_mixer_tank_res: [&'a str; 8],
    pub tx_pll_bw:         [&'a str; 4],
    pub tx_filter_bw:      [&'a str; 16],
    pub tx_dac_bw:         [&'a str; 8],
    pub rx_lna_gain:       [&'a str; 8],
    pub rx_pga_gain:       [&'a str; 16],
    pub rx_zin_200:        [&'a str; 2],
    pub rx_adc_bw:         [&'a str; 8],
    pub rx_pga_bw:         [&'a str; 4],
    pub rx_pll_bw:         [&'a str; 4],
    pub iomap0:            [&'a str; 4],
    pub iomap1:            [&'a str; 4],
    pub iomap2:            [&'a str; 4],
    pub iomap3:            [&'a str; 4],
    pub ckout_enable:      [&'a str; 2],
    pub ck_select_tx_dac:  [&'a str; 2],
}

pub const OPTS: SettingOptions<'static> = SettingOptions {
    tx_dac_gain: [
        "maximum gain - 9 dB",
        "maximum gain - 6 dB",
        "maximum gain - 3 dB",
        "maximum gain (0 dB full scale)",
        "Max gain - 9 dB with test Vref voltage",
        "Max gain - 6 dB with test Vref voltage",
        "Max gain - 3 dB with test Vref voltage",
        "Max gain, 0 dBFS with test Vref voltage",
    ],
    tx_mixer_gain: [
        "-37.5 dB", "-35.5 dB", "-33.5 dB", "-31.5 dB", "-29.5 dB", "-27.5 dB",
        "-25.5 dB", "-23.5 dB", "-21.5 dB", "-19.5 dB", "-17.5 dB", "-15.5 dB",
        "-13.5 dB", "-11.5 dB", "-9.5 dB", "-7.5 dB",
    ],
    tx_mixer_tank_cap: [
        "0 fF", "128 fF", "256 fF", "384 fF", "512 fF", "640 fF", "768 fF",
        "896 fF",
    ],
    tx_mixer_tank_res: [
        "0.95 kΩ", "1.11 kΩ", "1.32 kΩ", "1.65 kΩ", "2.18 kΩ", "3.24 kΩ",
        "6.00 kΩ", "none => about 64 kΩ",
    ],
    tx_pll_bw: ["75 KHz", "150 KHz", "225 KHz", "300 KHz",],
    tx_filter_bw: [
        "0.418 Mhz", "0.429 Mhz", "0.440 Mhz", "0.451 Mhz", "0.464 Mhz",
        "0.476 Mhz", "0.490 Mhz", "0.504 Mhz", "0.520 Mhz", "0.546 Mhz",
        "0.553 Mhz", "0.572 Mhz", "0.591 Mhz", "0.613 Mhz", "0.635 Mhz",
        "0.660 Mhz",
    ],
    tx_dac_bw: [
        "24 taps", "32 taps", "40 taps", "48 taps", "56 taps", "64 taps",
        "not used", "not used",
    ],
    rx_lna_gain: [
        "not used", "G1 = highest gain power - 0 dB", "G2 = highest gain power - 6 dB",
        "G3 = highest gain power - 12 dB", "G4 = highest gain power - 24 dB",
        "G5 = highest gain power - 36 dB", "G6 = highest gain power - 48 dB", "not used",
    ],
    rx_pga_gain: [
        "lowest gain + 0 dB", "lowest gain + 2 dB", "lowest gain + 4 dB",
        "lowest gain + 6 dB", "lowest gain + 8 dB", "lowest gain + 10 db",
        "lowest gain + 12 dB", "lowest gain + 14 dB", "lowest gain + 16 dB",
        "lowest gain + 18 dB", "lowest gain + 20 dB", "lowest gain + 22 dB",
        "lowest gain + 24 dB", "lowest gain + 26 dB", "lowest gain + 28 dB",
        "lowest gain + 30 dB",
    ],
    rx_zin_200: ["50Ω", "200Ω",],
    rx_adc_bw: [
        "unused", "use 0x01 instead ???", "100 kHz < BW < 400 kHz", "unused",
        "unused", "200 kHz < BW < 400 kHz", "unused", "BW > 400 kHz",
    ],
    rx_pga_bw: [ "1500 kHz", "1000 kHz", "750 kHz", "500 kHz", ],
    rx_pll_bw: [ "75 KHz", "150 KHz", "225 KHz", "300 KHz", ],
    iomap0: [ "pll_lock_rx", "pll_lock_rx", "pll_lock_rx", "eol" ],
    iomap1: [ "pll_lock_tx", "not used", "not used", "not used" ],
    iomap2: [ "xosc_ready", "not used", "not used", "not used" ],
    iomap3: [
        "pll_lock_rx in Rx mode & pll_lock_tx in all other modes", "not used",
        "not used", "not used" 
    ],
    ckout_enable: [
        "output clock disabled on pad CLK_OUT",
        "output clock enabled on pad CLK_OUT",
    ],
    ck_select_tx_dac: [
        "internal clock (CLK_XTAL) used for Tx DAC",
        "external clock (CLK_IN) used for Tx DAC",
    ],
};
/*
pub static CK_SELECT_TX_DAC_OPTS: [&str; 2] = [
    "internal clock (CLK_XTAL) used for Tx DAC",
    "external clock (CLK_IN) used for Tx DAC",
];
pub static EOL_OPTS: [&str; 2] = [
    "0 to VBAT > EOL threshold",
    "1 to VBAT < EOL threshold (battery low)",
];
pub static IISM_MODE_OPTS: [&str; 4] = ["mode A", "mode B1", "mode B2", "not used"];
pub static IISM_CLK_DIV_OPTS: [&str; 9] = [
    "1", "2", "4", "8", "12", "16", "24", "32", "48",
];
pub static IISM_TRUNCATION_OPTS: [&str; 2] = [
    "MSB is truncated, alignment on LSB",
    "LSB is truncated, alignment on MSB",
];
pub static IISM_STATUS_FLAG_OPTS: [&str; 2] = ["no error", "error, IISM off"]; */
