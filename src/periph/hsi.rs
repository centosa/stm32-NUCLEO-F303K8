//! 8MHz Internal RC oscillator clock.

use drone_core::periph;

periph::singular! {
    /// Extracts HSI register tokens.
    pub macro periph_hsi;

    /// HSI peripheral.
    pub struct HsiPeriph;

    drone_stm32_map::reg;
    crate::periph::hsi;

    RCC {
        CR {
            HSION;
            HSIRDY;
        }
    }
}
