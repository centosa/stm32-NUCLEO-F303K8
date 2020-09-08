//! GPIO pins bindings.

use crate::drv::gpio::GpioHeadEn;

use drone_core::inventory;
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::{
    head::GpioBHead,
    pin::{GpioB4, GpioB5, GpioPinPeriph},
};

/// Acquires [`GpioPins`].
#[doc(hidden)]
#[macro_export]
macro_rules! drv_gpio_pins {
    ($reg:ident) => {
        $crate::sys::gpio_pins::GpioPins::new($crate::sys::gpio_pins::GpioPinsRes {
            gpio_b4: ::drone_stm32_map::periph::gpio::periph_gpio_b4!($reg),
            gpio_b5: ::drone_stm32_map::periph::gpio::periph_gpio_b5!($reg),
        })
    };
}

/// GPIO pins driver.
pub struct GpioPins(GpioPinsRes);

/// GPIO pins resource for driving the LEDs on the NUCLEO.
pub struct GpioPinsRes {
    /// LED.
    pub gpio_b4: GpioPinPeriph<GpioB4>,
    /// Virtual user button.
    pub gpio_b5: GpioPinPeriph<GpioB5>,
}

impl GpioPins {
    /// Creates a new [`GpioPins`].
    #[inline]
    pub fn new(res: GpioPinsRes) -> Self {
        Self(res)
    }

    /// Releases resources.
    #[inline]
    pub fn free(self) -> GpioPinsRes {
        self.0
    }

    /// Initializes GPIO pins.
    pub fn init(
        &self,
        _gpio_b_en: &inventory::Token<GpioHeadEn<GpioBHead>>,
    ) {
        self.0.gpio_b4.gpio_moder_moder.modify(|r| {
            self.0.gpio_b4.gpio_moder_moder.write(r, 0b01);
        });
        self.0.gpio_b4.gpio_otyper_ot.modify(|r| {
            self.0.gpio_b4.gpio_otyper_ot.clear(r);
        });
        self.0.gpio_b4.gpio_ospeedr_ospeedr.modify(|r| {
            self.0.gpio_b4.gpio_ospeedr_ospeedr.write(r, 0b00);
        });
        self.0.gpio_b4.gpio_pupdr_pupdr.modify(|r| {
            self.0.gpio_b4.gpio_pupdr_pupdr.write(r, 0b00);
        });
        // -------------
        self.0.gpio_b5.gpio_moder_moder.modify(|r| {
            self.0.gpio_b5.gpio_moder_moder.write(r, 0b00); // Input
        });
        self.0.gpio_b5.gpio_pupdr_pupdr.modify(|r| {
            self.0.gpio_b5.gpio_pupdr_pupdr.write(r, 0b10);
        });
    }

    /// Sets the output `value` for the `pin`.
    pub fn output(
        &self,
        pin: u8,
        value: bool,
    ) {
        match pin {
            1 => {
                if value {
                    self.0.gpio_b4.gpio_bsrr_bs.set_bit();
                } else {
                    self.0.gpio_b4.gpio_bsrr_br.set_bit();
                }
            }
            _ => panic!("invalid gpio pin"),
        }
    }
}
