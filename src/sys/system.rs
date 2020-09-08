//! System associated helper functions.

use crate::consts::{HSE_CLK, HSI_CLK};
use crate::tasks::root::SystemRes;
//use crate::thr;
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
//use drone_stm32_map::periph::sys_tick::SysTickPeriph;
use drone_core::log;
use drone_cortexm::swo;
use futures::prelude::*;

/// An error returned when a receiver has missed too many ticks.
#[derive(Debug)]
pub struct TickOverflow;

/// System.
pub struct System {}

impl System {
    /// Creates a new [`System`].
    #[inline]
    pub fn new() -> Self {
        Self {}
    }

    /// Apply the current clock tree configuration.
    pub fn apply_clock_config(res: &SystemRes) {
        res.flash.set_latency(2);
        res.hsi.init(res);
        // Start pll only if used as clock source.
        if res.clksrc == 0b10 {
            res.pll.init(res);
            swo::update_prescaler((HSI_CLK/2)*(res.pllmul+2) / log::baud_rate!() - 1);
            System::delay(50, System::calculate_hclk(res), res).root_wait();
            res.pll.enable();
        }
        res.rcc.init(res);
        res.flash.set_latency(System::calculate_latency(res));
    }

    /// Resets the RCC.
    pub fn reset_rcc(res: &SystemRes) {
        res.rcc.reset();
        res.pll.disable();
        res.pll.reset();
        res.hsi.reset();
        swo::update_prescaler(HSI_CLK / log::baud_rate!() - 1);
        System::delay(50, System::calculate_hclk(res), res).root_wait();
    }

    /// Set flash read access latency.
    // To correctly read data from Flash memory, the number of
    // wait states (LATENCY) must be correctly programmed
    pub fn calculate_latency(res: &SystemRes) -> u32 {
        let mut hclk: u32;
        println!("SWS field value config: {}",res.clksrc);
        match res.clksrc {
            0b00 => {
                // HSI oscillator used as system clock.
                hclk = HSI_CLK;
            }
            0b01 => {
                // HSE oscillator used as system clock.
                hclk = HSE_CLK;
            }
            0b10 => {
                // PLL used as system clock.
                println!("PLLSRC value config: {}",res.pllsrc);
                match res.pllsrc {
                    0b00 => {
                        // HSI/2 selected as PLL input clock. 
                        hclk = HSI_CLK >> 1;
                        println!("PLL input: {} MHz", hclk);
                    }
                    0b01=> {
                        // HSE/PREDIV selected as PLL input clock 
                        hclk = HSE_CLK / (res.prediv + 1);
                    }
                    _ => {
                        // No clock sent to PLL.
                        hclk = 0;
                    }
                }
                // Multiply by decoded value of main PLL multiplication factor.
                println!("res.pllmul + 2 = {}", res.pllmul + 2);
                hclk = hclk * (res.pllmul + 2); 
            }
            _ => hclk = HSI_CLK,
        }

        // Return the correct number of wait states according to ref manual.
        println!("hclk for latency {}", hclk);
        if hclk <= 24_000_000 {
            0b00
        }
        else if hclk <= 48_000_000 {
            0b01
        }
        else {
            0b10
        }
    }

    pub fn calculate_hclk(res: &SystemRes) -> u32 {
        let mut hclk: u32;
        // Check which clock source is used as system clock.
        println!("SWS field value: {}",res.rcc.read_sws());
        match res.rcc.read_sws() {
            0b00 => {
                // HSI oscillator used as system clock.
                hclk = HSI_CLK;
            }
            0b01 => {
                // HSE used as system clock.
                hclk = HSE_CLK;
            }
            0b10 => {
                // PLL used as system clock.
                match res.pll.read_pllsrc() {
                    0b00 => {
                        // HSI/2 selected as PLL input clock. 
                        hclk = HSI_CLK / 2;
                    }
                    0b01 => {
                        // HSE/PREDIV selected as PLL input clock 
                        hclk = HSE_CLK / (res.prediv + 1);
                    }
                    _ => {
                        // No clock sent to PLL, PLLSAI1 and PLLSAI2
                        hclk = 0;
                    }
                }
                // Multiply by value of main PLL multiplication factor.
                println!("PLLMUL field value: {}",res.pll.read_pllmul());
                hclk = hclk * (res.pll.read_pllmul() + 2);
            }
            _ => hclk = HSI_CLK,
        }
        hclk
    }

    /// Millisecond delay.
    pub async fn delay(
        millis: u32,
        hclk: u32,
        res: &SystemRes,
        //sys_tick: &SysTickPeriph,
        //thr_sys_tick: thr::SysTick,
    ) -> () {
        let mut tick_stream = res.thr_sys_tick
            .add_pulse_try_stream(|| Err(TickOverflow), fib::new_fn(|| fib::Yielded(Some(1))));
        res.sys_tick.stk_val.store(|r| r.write_current(0));
        res.sys_tick
            .stk_load
            .store(|r| r.write_reload(millis * (hclk / 8000)));
        res.sys_tick.stk_ctrl.store(|r| {
            r.set_tickint() // Counting down to 0 triggers the SysTick interrupt
                .set_enable() // Start the counter in a multi-shot way
        });
        tick_stream.next().await;
    }
}
