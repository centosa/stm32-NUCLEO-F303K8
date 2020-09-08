//! Drivers common traits.

#[cfg(all(
    feature = "dma",
    any(
        stm32_mcu = "stm32l4r5",
        stm32_mcu = "stm32l4r7",
        stm32_mcu = "stm32l4r9",
        stm32_mcu = "stm32l4s5",
        stm32_mcu = "stm32l4s7",
        stm32_mcu = "stm32l4s9"
    )
))]
use crate::dma::mux::DmamuxChEn;
#[cfg(feature = "dma")]
use crate::dma::DmaChEn;
#[allow(unused_imports)]
use drone_cortexm::thr::prelude::*;
#[cfg(feature = "dma")]
use drone_stm32_map::periph::dma::ch::DmaChMap;

/// Driver reset and clock control.
pub trait DrvRcc {
    /// Resets the peripheral.
    fn reset(&mut self);

    /// Disables the peripheral clocks by the clock gating during Sleep and Stop
    /// modes.
    fn disable_stop_mode(&self);

    /// Enables the peripheral clocks by the clock gating during Sleep and Stop
    /// modes.
    fn enable_stop_mode(&self);
}

/// Driver clock source selection.
pub trait DrvClockSel {
    /// Selects a clock source for the peripheral.
    fn clock_sel(&self, value: u32);
}

/// Driver DMA receiver.
#[cfg(feature = "dma")]
pub trait DrvDmaRx<Rx: DmaChMap> {
    /// Initializes peripheral address of the DMA channel to the receiver.
    fn dma_rx_paddr_init(&self, dma_rx: &DmaChEn<Rx, impl IntToken>);

    #[cfg(any(
        stm32_mcu = "stm32l4r5",
        stm32_mcu = "stm32l4r7",
        stm32_mcu = "stm32l4r9",
        stm32_mcu = "stm32l4s5",
        stm32_mcu = "stm32l4s7",
        stm32_mcu = "stm32l4s9"
    ))]
    /// Initializes the DMA channel as a receiver.
    fn dma_rx_init(
        &self,
        dma_rx: &DmaChEn<Rx, impl IntToken>,
        dmamux_rx: &DmamuxChEn<Rx::DmamuxChMap>,
        rx_dma_req_id: u32,
    ) {
        self.dma_rx_paddr_init(dma_rx);
        dmamux_rx.set_dma_req_id(rx_dma_req_id);
    }

    #[cfg(any(
        stm32_mcu = "stm32l4x1",
        stm32_mcu = "stm32l4x2",
        stm32_mcu = "stm32l4x3",
        stm32_mcu = "stm32l4x5",
        stm32_mcu = "stm32l4x6"
    ))]
    /// Initializes the DMA channel as a receiver.
    fn dma_rx_init(&self, dma_rx: &DmaChEn<Rx, impl IntToken>, dma_rx_ch: u32) {
        self.dma_rx_paddr_init(dma_rx);
        dma_rx.ch_select(dma_rx_ch);
    }

    #[cfg(not(any(
        stm32_mcu = "stm32l4x1",
        stm32_mcu = "stm32l4x2",
        stm32_mcu = "stm32l4x3",
        stm32_mcu = "stm32l4x5",
        stm32_mcu = "stm32l4x6",
        stm32_mcu = "stm32l4r5",
        stm32_mcu = "stm32l4r7",
        stm32_mcu = "stm32l4r9",
        stm32_mcu = "stm32l4s5",
        stm32_mcu = "stm32l4s7",
        stm32_mcu = "stm32l4s9"
    )))]
    /// Initializes the DMA channel as a receiver.
    fn dma_rx_init(&self, dma_rx: &DmaChEn<Rx, impl IntToken>) {
        self.dma_rx_paddr_init(dma_rx);
    }
}

/// Driver DMA transmitter.
#[cfg(feature = "dma")]
pub trait DrvDmaTx<Tx: DmaChMap> {
    /// Initializes peripheral address of the DMA channel to the transmitter.
    fn dma_tx_paddr_init(&self, dma_tx: &DmaChEn<Tx, impl IntToken>);

    #[cfg(any(
        stm32_mcu = "stm32l4r5",
        stm32_mcu = "stm32l4r7",
        stm32_mcu = "stm32l4r9",
        stm32_mcu = "stm32l4s5",
        stm32_mcu = "stm32l4s7",
        stm32_mcu = "stm32l4s9"
    ))]
    /// Initializes the DMA channel as a transmitter.
    fn dma_tx_init(
        &self,
        dma_tx: &DmaChEn<Tx, impl IntToken>,
        dmamux_tx: &DmamuxChEn<Tx::DmamuxChMap>,
        tx_dma_req_id: u32,
    ) {
        self.dma_tx_paddr_init(dma_tx);
        dmamux_tx.set_dma_req_id(tx_dma_req_id);
    }

    #[cfg(any(
        stm32_mcu = "stm32l4x1",
        stm32_mcu = "stm32l4x2",
        stm32_mcu = "stm32l4x3",
        stm32_mcu = "stm32l4x5",
        stm32_mcu = "stm32l4x6"
    ))]
    /// Initializes the DMA channel as a transmitter.
    fn dma_tx_init(&self, dma_tx: &DmaChEn<Tx, impl IntToken>, dma_tx_ch: u32) {
        self.dma_tx_paddr_init(dma_tx);
        dma_tx.ch_select(dma_tx_ch);
    }

    #[cfg(not(any(
        stm32_mcu = "stm32l4x1",
        stm32_mcu = "stm32l4x2",
        stm32_mcu = "stm32l4x3",
        stm32_mcu = "stm32l4x5",
        stm32_mcu = "stm32l4x6",
        stm32_mcu = "stm32l4r5",
        stm32_mcu = "stm32l4r7",
        stm32_mcu = "stm32l4r9",
        stm32_mcu = "stm32l4s5",
        stm32_mcu = "stm32l4s7",
        stm32_mcu = "stm32l4s9"
    )))]
    /// Initializes the DMA channel as a transmitter.
    fn dma_tx_init(&self, dma_tx: &DmaChEn<Tx, impl IntToken>) {
        self.dma_tx_paddr_init(dma_tx);
    }
}
