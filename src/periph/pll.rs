//! Phase-Locked Loop clock.

use drone_core::periph;

periph::singular! {
    /// Extracts PLL register tokens.
    pub macro periph_pll;

    /// PLL peripheral.
    pub struct PllPeriph;

    drone_stm32_map::reg;
    crate::periph::pll;

    RCC {
        CR {
            PLLON;
            PLLRDY;
        }
        CFGR {
            PLLSRC; 
            PLLMUL; 
        }
    }
}

