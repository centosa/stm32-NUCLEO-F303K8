//! Embedded Flash memory.

use drone_core::periph;

periph::singular! {
    /// Extracts Flash register tokens.
    pub macro periph_flash;

    /// Flash peripheral.
    pub struct FlashPeriph;

    drone_stm32_map::reg;
    crate::periph::flash;

    FLASH {
        ACR;
    }
}
