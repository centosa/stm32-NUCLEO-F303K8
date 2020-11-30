//! General-purpose I/O utility trait.

use crate::drv::common::DrvRcc;
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::head::{GpioHeadMap, GpioHeadPeriph};
use drone_core::{inventory, inventory::Inventory};
use typenum::{U0, U1};

/// GPIO port head driver.
//pub struct GpioHead<T: GpioHeadMap>(Inventory<GpioHeadEn<T>, 0>);
pub struct GpioHead<T: GpioHeadMap>(Inventory<GpioHeadEn<T>, U0>);

/// GPIO port head enabled driver.
pub struct GpioHeadEn<T: GpioHeadMap> {
    periph: GpioHeadPeriph<T>,
}

impl<T: GpioHeadMap> GpioHead<T> {
    /// Creates a new [`GpioHead`].
    #[inline]
    pub fn new(periph: GpioHeadPeriph<T>) -> Self {
        Self(Inventory::new(GpioHeadEn { periph }))
    }

    /// Releases the peripheral.
    #[inline]
    pub fn free(self) -> GpioHeadPeriph<T> {
        Inventory::free(self.0).periph
    }

    /// Enables the port clock.
    pub fn enable(&mut self) -> inventory::Guard<'_, GpioHeadEn<T>> {
        self.setup();
        Inventory::guard(&mut self.0)
    }

    /// Enables the port clock.
    pub fn into_enabled(self) -> Inventory<GpioHeadEn<T>, U1> {
        self.setup();
        let (enabled, token) = self.0.share1();
        // To be recreated in `from_enabled()`.
        drop(token);
        enabled
    }

    /// Disables the port clock.
    pub fn from_enabled(enabled: Inventory<GpioHeadEn<T>, U1>) -> Self {
        // Restoring the token dropped in `into_enabled()`.
        let token = unsafe { inventory::Token::new() };
        let mut enabled = enabled.merge1(token);
        Inventory::teardown(&mut enabled);
        Self(enabled)
    }

    fn setup(&self) {
        let gpioen = &self.0.periph.rcc_busenr_gpioen;
        if gpioen.read_bit() {
            panic!("GPIO wasn't turned off");
        }
        gpioen.set_bit();
    }
}

impl<T: GpioHeadMap> DrvRcc for GpioHead<T> {
    #[inline]
    fn reset(&mut self) {
        self.0.reset();
    }

    #[inline]
    fn disable_stop_mode(&self) {
        self.0.disable_stop_mode();
    }

    #[inline]
    fn enable_stop_mode(&self) {
        self.0.enable_stop_mode();
    }
}

impl<T: GpioHeadMap> inventory::Item for GpioHeadEn<T> {
    fn teardown(&mut self, _token: &mut inventory::GuardToken<Self>) {
        self.periph.rcc_busenr_gpioen.clear_bit()
    }
}

impl<T: GpioHeadMap> DrvRcc for GpioHeadEn<T> {
    fn reset(&mut self) {
        self.periph.rcc_busrstr_gpiorst.set_bit();
    }

    fn disable_stop_mode(&self) {
        self.periph.rcc_busenr_gpioen.clear_bit();
    }

    fn enable_stop_mode(&self) {
        self.periph.rcc_busenr_gpioen.set_bit();
    }
}
