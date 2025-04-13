use std::fs::{File, read_to_string};
use std::path::PathBuf;
use std::io::Write;
use chrono::prelude::*;
use serde::Deserialize;

use crate::SX1255Info;

pub fn write_file(sx1255_info: SX1255Info, filename: &PathBuf) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    write!(file, "
# SX1255 configuration
# created by sx1255-config on {datetime}

[general]
driver_enable = {driver_enable}
tx_enable = {tx_enable}
rx_enable = {rx_enable}
ref_enable = {ref_enable}
rx_freq = {rx_freq}
tx_freq = {tx_freq}

[tx]
# DAC gain programmable in 3 dB steps:
# 0 (maximum gain - 9 dB)
# 1 (maximum gain - 6 dB)
# 2 (maximum gain - 3 dB)
# 3 (maximum gain (0 dB full scale))
# Test modes, not recommended:
# 4 (Max gain - 9 dB with test Vref voltage)
# 5 (Max gain - 6 dB with test Vref voltage)
# 6 (Max gain - 3 dB with test Vref voltage)
# 7 (Max gain, 0 dBFS with test Vref voltage)
tx_dac_gain = {tx_dac_gain}

# Gain ~ -37.5 + 2 x tx_mixer_gain in dB
tx_mixer_gain = {tx_mixer_gain} # 0-15

# Cap = 128 * tx_mixer_tank_cap [fF]
tx_mixer_tank_cap = {tx_mixer_tank_cap} # 0-7

# 0=0.95kΩ, 1=1.11kΩ, 2=1.32kΩ, 3=1.65kΩ, 4=2.18Ω
# 5=3.24kΩ, 6=6.00kΩ, 7=none => about 64kΩ
tx_mixer_tank_res = {tx_mixer_tank_res}

# PLL BW = (tx_pll_bw + 1)*75 KHz
tx_pll_bw = {tx_pll_bw} # 0-3

# BW3dB = 17.15 / (41 - tx_filter_bw) MHz
tx_filter_bw = {tx_filter_bw} # 0-15

# Number of taps of FIR-DAC: Actual number of taps = 24 + 8.tx_dac_bw, max=64
tx_dac_bw = {tx_dac_bw}

[rx]
# 0 - not used
# 1 - G1, highest gain power - 0 dB
# 2 - G2, highest gain power - 6 dB
# 3 - G3, highest gain power - 12 dB
# 4 - G4, highest gain power - 24 dB
# 5 - G5, highest gain power - 36 dB
# 6 - G6, highest gain power - 48 dB
# 7 - not used
rx_lna_gain = {rx_lna_gain}

# Gain=lowest gain + 2dB * rx_pga_gain
rx_pga_gain = {rx_pga_gain} # 0-7

# 0=50Ω, 1=200Ω
rx_zin_200 = {rx_zin_200}

# For BW>400kHz SSB use 7
# For 200kHz< BW<400kHz SSB use 5
# For 100kHz<BW<400kHz SSB use 2
# use 1 instead (???)
rx_adc_bw = {rx_adc_bw}

rx_adc_trim = {rx_adc_trim} # 0-7

# 0=1500 kHz
# 1=1000 kHz
# 2=750 kHz
# 3=500 kHz 
rx_pga_bw = {rx_pga_bw}

# PLL BW = (rx_pll_bw + 1)*75 KHz 
rx_pll_bw = {rx_pll_bw} # 0-3

rx_adc_temp = {rx_adc_temp}

[iomap]
# 0=pll_lock_rx, 1=pll_lock_rx, 2=pll_lock_rx, 3=eol
iomap0 = {iomap0}

# 0=pll_lock_tx
iomap1 = {iomap1}

# 0=xosc_ready
iomap2 = {iomap2}

# 0=pll_lock_rx in Rx mode & pll_lock_tx in all other modes
iomap3 = {iomap3}

[additional]
dig_loopback_en = {dig_loopback_en}
rf_loopback_en = {rf_loopback_en}

# 0: output clock disabled on pad CLK_OUT
# 1: output clock enabled on pad CLK_OUT
ckout_enable = {ckout_enable}

# 0: internal clock (CLK_XTAL) used for Tx DAC
# 1: external clock (CLK_IN) used for Tx DAC
ck_select_tx_dac = {ck_select_tx_dac}

iism_rx_disable = {iism_rx_disable}
iism_tx_disable = {iism_tx_disable}

# 0=mode A, 1=mode B1, 2=mode B2, 3=not used
iism_mode = {iism_mode}

# 0=1, 1=2, 2=4, 3=8, 4=12, 5=16, 6=24, 7=32 
iism_clk_div = {iism_clk_div}

# decimation/interpolation factor, valid values:
# set 1: 8, 16, 24, 32, 48, 64, 96, 128, 192, 256, 384, 512, 768, 1536
# set 2: 9, 18, 27, 36, 54, 72, 108, 144, 216, 288, 432, 576, 864, 1728
r = {r}
",
        datetime          = Utc::now(),
        driver_enable     = sx1255_info.driver_enable,
        tx_enable         = sx1255_info.tx_enable,
        rx_enable         = sx1255_info.rx_enable,
        ref_enable        = sx1255_info.ref_enable,
        rx_freq           = sx1255_info.rx_freq,
        tx_freq           = sx1255_info.tx_freq,
        tx_dac_gain       = sx1255_info.tx_dac_gain,
        tx_mixer_gain     = sx1255_info.tx_mixer_gain,
        tx_mixer_tank_cap = sx1255_info.tx_mixer_tank_cap,
        tx_mixer_tank_res = sx1255_info.tx_mixer_tank_res,
        tx_pll_bw         = sx1255_info.tx_pll_bw,
        tx_filter_bw      = sx1255_info.tx_filter_bw,
        tx_dac_bw         = sx1255_info.tx_dac_bw,
        rx_lna_gain       = sx1255_info.rx_lna_gain,
        rx_pga_gain       = sx1255_info.rx_pga_gain,
        rx_zin_200        = sx1255_info.rx_zin_200,
        rx_adc_bw         = sx1255_info.rx_adc_bw,
        rx_adc_trim       = sx1255_info.rx_adc_trim,
        rx_pga_bw         = sx1255_info.rx_pga_bw,
        rx_pll_bw         = sx1255_info.rx_pll_bw,
        rx_adc_temp       = sx1255_info.rx_adc_temp,
        iomap0            = sx1255_info.iomap0,
        iomap1            = sx1255_info.iomap1,
        iomap2            = sx1255_info.iomap2,
        iomap3            = sx1255_info.iomap3,
        dig_loopback_en   = sx1255_info.dig_loopback_en,
        rf_loopback_en    = sx1255_info.rf_loopback_en,
        ckout_enable      = sx1255_info.ckout_enable,
        ck_select_tx_dac  = sx1255_info.ck_select_tx_dac,
        iism_rx_disable   = sx1255_info.iism_rx_disable,
        iism_tx_disable   = sx1255_info.iism_tx_disable,
        iism_mode         = sx1255_info.iism_mode,
        iism_clk_div      = sx1255_info.iism_clk_div,
        r                 = sx1255_info.r,
    )?;

    Ok(())
}

pub fn read_file(sx1255_info: &mut SX1255Info, filename: &PathBuf) -> std::io::Result<()> {

    #[derive(Debug, Deserialize)]
    struct SX1255Config {
        #[allow(dead_code)]
        general: General,
        /*#[allow(dead_code)]
        rx: Rx,
        #[allow(dead_code)]
        tx: Tx,
        #[allow(dead_code)]
        iomap: Iomap,
        #[allow(dead_code)]
        additional: Additional,*/
    }

    #[derive(Debug, Deserialize)]
    struct General {
        #[allow(dead_code)]
        driver_enable: bool,
        #[allow(dead_code)]
        tx_enable: bool,
        #[allow(dead_code)]
        rx_enable: bool,
        #[allow(dead_code)]
        ref_enable: bool,
    }

    let content = read_to_string(filename)?;
    let config: SX1255Config = toml::from_str(&content).expect("Failed to deserialize config file");

    sx1255_info.driver_enable = config.general.driver_enable;
    sx1255_info.tx_enable     = config.general.tx_enable;
    sx1255_info.rx_enable     = config.general.rx_enable;
    sx1255_info.ref_enable    = config.general.ref_enable;

    Ok(())
}
