//! Project constants.

/// SWO baud rate.
pub const SWO_BAUD_RATE: usize = 115_200;

// HSI internal 8 MHz RC Oscillator.
pub const HSI_CLK: u32 = 8_000_000;

// HSE high speed external clock (not present on Nucleo-144)
pub const HSE_CLK: u32 = 48_000_000;
