//! Reset and Clock Control.

use crate::periph::rcc::RccPeriph;
use crate::tasks::root::SystemRes;
use drone_cortexm::reg::prelude::*;

/// RCC driver.
pub struct Rcc {
    periph: RccPeriph,
}

impl Rcc {
    /// Creates a new [`Rcc`].
    #[inline]
    pub fn new(periph: RccPeriph) -> Self {
        Self { periph }
    }

    /// Releases the peripheral.
    #[inline]
    pub fn free(self) -> RccPeriph {
        self.periph
    }

    /// Initializes RCC.
    #[inline]
    pub fn init(&self, res: &SystemRes) {
        self.periph.rcc_cfgr_sw.write_bits(res.clksrc);
        self.periph.rcc_cfgr_hpre.write_bits(res.hpre);
    }

    /// Reset RCC to default.
    pub fn reset(&self) {
        self.periph.rcc_cfgr_sw.write_bits(0b00);
        self.periph.rcc_cfgr_hpre.write_bits(0b0000);
        self.periph.rcc_cfgr_ppre1.write_bits(0b000);
        self.periph.rcc_cfgr_ppre2.write_bits(0b000);
    }

    /// Read the system clock switch status from mcu.
    pub fn read_sws(&self) -> u32 {
        self.periph.rcc_cfgr_sws.read_bits() as u32
    }

        /// Power interface clock enable.
    #[inline]
    pub fn set_apb1enr_pwren(&self) -> () {
        self.periph.rcc_apb1enr.modify(|r| r.set_pwren());
    }

    /// Power interface clock disable.
    #[inline]
    pub fn clear_apb1enr_pwren(&self) -> () {
        self.periph.rcc_apb1enr.modify(|r| r.clear_pwren());
    }

    /// Disable backup domain write protection
    #[inline]
    pub fn set_pwr_cr_dbp(&self) {
        self.periph.pwr_cr_dbp.set_bit();
    }

}
