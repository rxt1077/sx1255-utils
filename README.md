# sx1255-utils

Utilities for working with the [M17 SX1255 HAT](https://github.com/M17-Project/SX1255_HAT-hw).

Based on the work of Wojciech Kaczmarski SP5WWP: [sx1255-spi.c](https://gist.github.com/sp5wwp/25fa989ebd98b3b707eadae9b63af679), [zmq-pub.c](https://gist.github.com/sp5wwp/c53602549f8ccde0c6e30d593aa6bb5b), and [zmq-sub.c](https://gist.github.com/sp5wwp/2df5b794c793be340941681a36e59918).

## sx1255-config

```
ryan@sx1255:~/sx1255-utils $ ./target/debug/sx1255-config --help
Configure the M17 sx1255 HAT via SPI/GPIO

Usage: sx1255-config <COMMAND>

Commands:
  info   Prints info about device state
  reset  Resets the device
  save   Save device state to file
  load   Loads device state from file
  set    Sets a register variable
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
ryan@sx1255:~/sx1255-utils $ ./target/debug/sx1255-config set --help
Sets a register variable

Usage: sx1255-config set <COMMAND>

Commands:
  driver_enable      Enables the PA driver
  tx_enable          Enables the complete Tx part of the front-end (except the PA)
  rx_enable          Enables the complete Rx part of the front-end
  ref_enable         Enables the PDS and the oscillator
  rx_freq            Sets the Rx frequency
  tx_freq            Sets the Tx frequency
  tx_dac_gain        Sets the Tx DAC gain
  tx_mixer_gain      Sets the Tx mixer gain
  tx_mixer_tank_cap  Sets the capacitance in parallel with the mixer tank
  tx_mixer_tank_res  Sets the resistance inparallel with the mixer tank
  tx_pll_bw          Sets Tx PLL bandwidth
  tx_filter_bw       Sets Tx analog filter bandwidth DSB
  tx_dac_bw          Sets the number of taps for the Tx FIR-DAC
  rx_lna_gain        Sets the Rx LNA gain
  rx_pga_gain        Sets the Rx PGA gain
  rx_zin200          Sets the Rx input impedance
  rx_adc_trim        Sets the Rx ADC trim for 36 MHz reference crystal (must be between 0-7)
  rx_pga_bw          Sets the Rx analog roofing filter
  rx_pll_bw          Sets the Rx PLL bandwidth
  rx_adc_temp        Puts the Rx ADC into temperature measurement mode
  io_map0            Mapping of DIO(0)
  io_map1            Mapping of DIO(1)
  io_map2            Mapping of DIO(2)
  io_map3            Mapping of DIO(3)
  dig_loopback_en    Enables the digital loop back mode of the frontend
  rf_loopback_en     Enables the RF loop back mode of the frontend
  iism_rx_disable    Disable IISM Rx (during Tx mode)
  iism_tx_disable    Disable IISM Tx (during Rx mode)
  iism_mode          Sets the IISM mode
  iism_clk_div       Sets the XTAL/CLK_OUT division factor
  r                  Sets the interpolation/decimation factor
  iism_truncation    Sets the IISM truncation mode
  help               Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
