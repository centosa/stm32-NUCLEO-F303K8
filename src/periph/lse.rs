//! 32.768 kHz Low Speed External resonator.

use drone_core::periph;

periph::singular! {
    /// Extracts LSE register tokens.
    pub macro periph_lse;

    /// LSE peripheral.
    pub struct LsePeriph;

    drone_stm32_map::reg;
    crate::periph::lse;

    RCC {
        BDCR {
            LSEBYP;
            LSEON;
            LSERDY;
            LSEDRV;
        }
    }
}
