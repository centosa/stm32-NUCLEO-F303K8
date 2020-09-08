//! 8MHz internal RC oscillator clock.

use crate::periph::hsi::HsiPeriph;
use crate::tasks::root::SystemRes;
use drone_cortexm::reg::prelude::*;

/// HSI driver.
pub struct Hsi {
    periph: HsiPeriph,
}

impl Hsi {
    /// Creates a new [`Hsi`].
    #[inline]
    pub fn new(periph: HsiPeriph) -> Self {
        Self { periph }
    }

    /// Releases the peripheral.
    #[inline]
    pub fn free(self) -> HsiPeriph {
        self.periph
    }

    /// Initializes HSI.
    pub fn init(&self, _res: &SystemRes) {
        println!("HSI init");
        self.periph.rcc_cr_hsion.modify(|r| {
            self.periph.rcc_cr_hsion.set(r);
        });
        while !self.periph.rcc_cr_hsirdy.read_bit_band() {}
    }

    /// Reset the HSI configuration to default
    pub fn reset(&self) {}
}
