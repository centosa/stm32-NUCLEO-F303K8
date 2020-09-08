//! 32.768 kHz Low Speed External resonator.

use crate::periph::lse::LsePeriph;
use crate::tasks::root::SystemRes;
use drone_cortexm::reg::prelude::*;

/// LSE driver.
pub struct Lse {
    periph: LsePeriph,
}

impl Lse {
    /// Creates a new [`Lse`].
    #[inline]
    pub fn new(periph: LsePeriph) -> Self {
        Self { periph }
    }

    /// Releases the peripheral.
    #[inline]
    pub fn free(self) -> LsePeriph {
        self.periph
    }

    /// Initializes LSE.
    pub fn init(&self, res: &SystemRes) {
        res.rcc.set_apb1enr_pwren();
        res.rcc.set_pwr_cr_dbp();
        self.periph.rcc_bdcr_lseon.modify(|r| {
            self.periph.rcc_bdcr_lseon.set(r);
            self.periph.rcc_bdcr_lsebyp.clear(r);
            self.periph.rcc_bdcr_lsedrv.write(r, 0b01);
        });
        res.rcc.clear_apb1enr_pwren();
        while !self.periph.rcc_bdcr_lserdy.read_bit_band() {}
    }

    pub fn reset(&self) {
        self.periph.rcc_bdcr_lseon.modify(|r| {
            self.periph.rcc_bdcr_lseon.clear(r);
        });
    }
}
