//! Reset and Clock Control.

use drone_core::periph;

periph::singular! {
    /// Extracts RCC register tokens.
    pub macro periph_rcc;

    /// RCC peripheral.
    pub struct RccPeriph;

    drone_stm32_map::reg;
    crate::periph::rcc;

    RCC {
        CFGR {
            SW;
            SWS;
            HPRE;
            PPRE1;
            PPRE2;
        }
        APB1ENR;
    }

    PWR {
        CR {
            DBP;
        }
    }

}
