//! Phase-Locked Loop clock.

use crate::periph::pll::PllPeriph;
use crate::tasks::root::SystemRes;
use drone_cortexm::reg::prelude::*;

/// PLL driver.
pub struct Pll {
    periph: PllPeriph,
}

impl Pll {
    /// Creates a new [`Pll`].
    #[inline]
    pub fn new(periph: PllPeriph) -> Self {
        Self { periph }
    }

    /// Releases the peripheral.
    #[inline]
    pub fn free(self) -> PllPeriph {
        self.periph
    }

    /// Initializes PLL.
    pub fn init(&self, res: &SystemRes) {
        self.periph.rcc_cfgr_pllsrc.write_bits(res.pllsrc);
        self.periph.rcc_cfgr_pllmul.write_bits(res.pllmul);
    }

    /// Enables PLL.
    pub fn enable(&self) {
        println!("Enable PLL");
        self.periph.rcc_cr_pllon.set_bit();
        while !self.periph.rcc_cr_pllrdy.read_bit() {}
        println!("PLL is enabled");
    }

    /// Disable PLL.
    pub fn disable(&self) {
        self.periph.rcc_cr_pllon.clear_bit();
        while self.periph.rcc_cr_pllrdy.read_bit() {}
    }

    /// Resets the PLL configuration to default.
    #[inline]
    pub fn reset(&self) {
        self.periph.rcc_cfgr_pllsrc.write_bits(0b00);
        self.periph.rcc_cfgr_pllmul.write_bits(0b0000);
    }

    /// Returns value of field PLLSRC.
    #[inline]
    pub fn read_pllsrc(&self) -> u32 {
        self.periph.rcc_cfgr_pllsrc.read_bits() as u32
    }

    /// Returns value of field PLLN.
    #[inline]
    pub fn read_pllmul(&self) -> u32 {
        self.periph.rcc_cfgr_pllmul.read_bits() as u32
    }

}
