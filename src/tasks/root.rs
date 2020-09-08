//! The root task.

use crate::consts::HSI_CLK;
use crate::{
    drv::{
        exti::{ExtiDrv, ExtiSetup},
        flash::Flash,
        gpio::GpioHead,
        hsi::Hsi,
        lse::Lse,
        pll::Pll,
        rcc::Rcc,
    },
    drv_gpio_pins,
    sys::{gpio_pins::GpioPins, system::System},
    thr,
    thr::{Thrs, ThrsInit},
    Regs,
};
use drone_core::log;
use drone_cortexm::swo;
use drone_cortexm::processor::fpu_init;
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::exti::periph_exti5;
use drone_stm32_map::periph::exti::Exti5;
use drone_stm32_map::periph::gpio::periph_gpio_b_head;
use drone_stm32_map::periph::sys_tick::{periph_sys_tick, SysTickPeriph};

use futures::prelude::*;
use futures::select_biased;

enum Event {
    Tick,
    Push,
}

enum ClockMode {
    Reset8MHz,
    Medium32MHz,
    High64MHz,
}

enum Led {
    GreenLed = 1,
}

/// An error returned when a receiver has missed too many ticks.
#[derive(Debug)]
pub struct TickOverflow;

/// System Resources
pub struct SystemRes {
    pub sys_tick: SysTickPeriph,
    pub thr_sys_tick: thr::SysTick,
    pub pll: Pll,
    pub hsi: Hsi,
    pub lse: Lse,
    pub rcc: Rcc,
    pub flash: Flash,
    pub pllmul: u32,
    pub clksrc: u32,
    pub pllsrc: u32,
    pub hpre: u32,
    pub prediv: u32,
}

#[allow(unused_labels)]
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let mut clock_mode = ClockMode::High64MHz;

    let (thr, scb) = thr::init_extended(thr_init);
    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    // Allocate the clock control resources.
    let mut res = SystemRes {
        sys_tick: periph_sys_tick!(reg),
        thr_sys_tick: thr.sys_tick,
        // ----------------------
        // -- Clocks.
        // The internal PLLs can be used to multiply the HSI or HSE
        // output clock frequency.
        pll: Pll::new(periph_pll!(reg)),
        // The HSI clock signal is generated from an internal 8 MHz RC Oscillator.
        hsi: Hsi::new(periph_hsi!(reg)),
        // The LSE clock (32.768K oscillator, not used in this crate.)
        lse: Lse::new(periph_lse!(reg)),
        // The RCC component.
        rcc: Rcc::new(periph_rcc!(reg)),
        // The flash component,
        flash: Flash::new(periph_flash!(reg)),
        // ----------------------
        // -- Factors and selectors.
        // CAUTION: Setting wrong values may make your system unusable.
        // Read the reference manual for detailed information.
        //
        // PLL multiplication factor.
        // Possible values for pllmul:
        // Caution: The PLL output frequency must not exceed 72 MHz.
        // 0000: PLL input clock x 2
        // 0001: PLL input clock x 3
        // 0010: PLL input clock x 4
        // 0011: PLL input clock x 5
        // 0100: PLL input clock x 6
        // 0101: PLL input clock x 7
        // 0110: PLL input clock x 8
        // 0111: PLL input clock x 9
        // 1000: PLL input clock x 10
        // 1001: PLL input clock x 11
        // 1010: PLL input clock x 12
        // 1011: PLL input clock x 13
        // 1100: PLL input clock x 14
        // 1101: PLL input clock x 15
        // 1110: PLL input clock x 16
        // 1111: Not applicable 
        pllmul: 0b1110,  // Field RCC_CFGR PLLMUL in ref. manual RM0316. 

        // System clock switch.
        // Possible values for clksrc:
        // 00: HSI oscillator used as system clock. 
        // 01: HSE oscillator used as system clock. 
        // 10: PLL used as system clock 
        // 11: Not applicable. 
        clksrc: 0b10, // Field RCC_CFGR SW in ref. manual RM0316.
        //
        // Possible values for pllsrc:
        // Caution: Different values for STM32F303xD/E and STM32F398xE!
        // 00: HSI/2 selected as PLL input clock. 
        // 01: HSE/PREDIV selected as PLL input clock 
        // 10: Reserved. 
        // 11: Reserved. 
        pllsrc: 0b00, // Field RCC_CFGR PLLSRC in ref. manual RM0316.

        // Division factor of the AHB clock (AHB prescaler).
        // Possible values for hpre:
        // 0xxx: SYSCLK not divided
        // 1000: SYSCLK divided by 2
        // 1001: SYSCLK divided by 4
        // 1010: SYSCLK divided by 8
        // 1011: SYSCLK divided by 16
        // 1100: SYSCLK divided by 64
        // 1101: SYSCLK divided by 128
        // 1110: SYSCLK divided by 256
        // 1111: SYSCLK divided by 512
        hpre: 0b0000, // Field RCC_CFGR HPRE in ref. manual RM0316.

        // PREDIV division factor.
        prediv: 0b000, // Field RCC_CFGR2 PREDIV in ref. manual RM0316.
    };

    swo::flush();
    swo::update_prescaler(HSI_CLK / log::baud_rate!() - 1);
    System::delay(100, HSI_CLK, &res).root_wait();

    // The on-board user LED is connected to GPIO bank B.
    // Create register and pins mapping component.
    let gpio_pins = drv_gpio_pins!(reg);
    let mut gpio_b = GpioHead::new(periph_gpio_b_head!(reg));
    // Enable and initialize.
    let gpio_b_en = gpio_b.enable();
    gpio_pins.init(gpio_b_en.inventory_token());

    scb.scb_ccr_div_0_trp.set_bit();
    unsafe {
        fpu_init(true);
    }

    // Enable the system configuration controller clock.
    reg.rcc_apb2enr.syscfgen.set_bit();

    // Setup fault handlers.
    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    // Exti configuration for the user button.
    // There is no user button on the Nucleo-F303K8,
    // but we use the PB4 pin to emulate it.
    let exti5 = ExtiDrv::init(ExtiSetup {
        exti: periph_exti5!(reg),
        exti_int: thr.exti9_5,
        config: 0b001,  // PB5 pin. 
        falling: false, // trigger the interrupt on a falling edge.
        rising: true,   // don't trigger the interrupt on a rising edge.
    });

    'user_button_pressed: loop {
        // Reset the clock control registers to their default.
        System::reset_rcc(&res);

        // Apply the current clock tree configuration.
        System::apply_clock_config(&res);

        // Calculate the configured clock speed.
        let hclk = System::calculate_hclk(&res);

        swo::flush();
        swo::update_prescaler(hclk / log::baud_rate!() - 1);
        System::delay(50, hclk, &res).root_wait();

        println!("Running at {} MHz", hclk);

        listen(&res, &thr, &exti5, &gpio_pins, hclk).root_wait();

        // Set different configuration for the clock tree
        // depending on current configuration
        match clock_mode {
            ClockMode::Reset8MHz => {
                clock_mode = ClockMode::Medium32MHz; // <- new mode.
                res.pllsrc = 0b00; // HSI is PLL clock input.
                res.clksrc = 0b10; // Use PLL output 32 MHz.
                res.pllmul = 0b0110;
                System::delay(50, 8_000_000, &res).root_wait();
            }
            ClockMode::Medium32MHz => {
                clock_mode = ClockMode::High64MHz; // <- new mode.
                res.pllsrc = 0b00; // HSI is PLL clock input.
                res.clksrc = 0b10; // Use PLL output 64 MHz
                res.pllmul = 0b1110;
                System::delay(50, 32_000_000, &res).root_wait();
            }
            ClockMode::High64MHz => {
                clock_mode = ClockMode::Reset8MHz; // <- new mode.
                res.pllsrc = 0b00; // No PLL.
                res.clksrc = 0b00; // Use HSI 8MHz.
                res.pllmul = 0b0000;
                System::delay(20, 64_000_0000, &res).root_wait();
            }
        }
    }
}

async fn listen(
    res: &SystemRes,
    thr: &Thrs,
    exti5: &ExtiDrv<Exti5, thr::Exti95>,
    gpio_pins: &GpioPins,
    hclk: u32,
) -> Event {
    println!("Enter listen, hclk={}", hclk);
    // Attach a listener that will notify us on user button pressed.
    let mut button_stream = exti5.create_saturating_stream();

    // Attach a listener that will notify us on each SYS_TICK interrupt trigger.
    let mut tick_stream = res.thr_sys_tick.add_pulse_try_stream(
        // This closure will be called when a receiver no longer can store the
        // number of ticks since the last stream poll. If this happens, a
        // `TickOverflow` error will be sent over the stream as is final value.
        || Err(TickOverflow),
        // A fiber that will be called on each interrupt trigger. It sends a
        // single tick over the stream.
        fib::new_fn(|| fib::Yielded(Some(1))),
    );

    // Clear the current value of the timer.
    res.sys_tick.stk_val.store(|r| r.write_current(0));
    //
    // The duration of setting the led ON is inversely proportional to the
    // MCU clock speed. It shall be:
    //   3.60 seconds when cpu clocks @ 4MHz
    //   0.40 seconds when cpu clocks @ 36MHz
    //   0.20 seconds when cpu clocks @ 72MHz

    // The trigger is set so that it returns twice per interval
    // at the highest speed, and proportionally more often per interval
    // at lower speeds.
    // That way, the Exti interrupt will happen every 100ms at all speeds
    // and it can be used to for debounceing and doubleclick control.
    let mut trigger = 4_000_000 / 8; // So many systick/sec at 4MHz.
    trigger = trigger / 10; // So many in 100ms at 4MHz.
    trigger = trigger * (hclk / 4_000_000); // More at higher speed

    res.sys_tick.stk_load.store(|r| r.write_reload(trigger));
    res.sys_tick.stk_ctrl.store(|r| {
        r.set_tickint() // Counting down to 0 triggers the SysTick interrupt
            .set_enable() // Start the counter in a multi-shot way
    });

    let mut green_led_on = true;
    gpio_pins.output(Led::GreenLed as u8, true); // Start with red led ON.

    // Enable the interrupt for the user button.
    thr.exti9_5.enable_int();

    // Counters
    let mut debounce_protection: i16 = 0;
    let mut doubleclick_protection: i16 = 0;
    let mut ticks_cnt: u32 = 0;

    // Monitored interval lengths (accumulated ticks).
    let debounce_ival = 2;
    let doubleclick_ival = 4;

    // This is dependent on mcu speed:
    let ticks_ival: u32 = 40 / (hclk / 4_000_000);

    'blinky: loop {
        let evt = select_biased! {
            p = button_stream.next().fuse() => Event::Push,
            t = tick_stream.next().fuse() => Event::Tick,
        };
        match evt {
            Event::Tick => {
                if debounce_protection > i16::MIN {
                    debounce_protection = debounce_protection - 1;
                };
                if doubleclick_protection < i16::MAX {
                    doubleclick_protection = doubleclick_protection + 1;
                };
                if debounce_protection == 0 && doubleclick_protection >= doubleclick_ival {
                    println!("Switch to new speed");
                    break 'blinky;
                }
                // The low and the high interval is 'ticks_ival' ticks.
                ticks_cnt = ticks_cnt + 1;
                if ticks_cnt >= ticks_ival {
                    ticks_cnt = 0;
                    match green_led_on {
                        true => {
                            println!("LED off");
                            green_led_on = false;
                            gpio_pins.output(Led::GreenLed as u8, false);
                        }
                        _ => {
                            println!("LED on");
                            green_led_on = true;
                            gpio_pins.output(Led::GreenLed as u8, true);
                        }
                    }
                }
            }
            Event::Push => {
                // After disabling the interrupt or after re-enabling 
                // the interrupt, the stream needs to be flushed to protect 
                // the logic during the switching period against mechanical 
                // contact bouncing and doubleclicks.
                if doubleclick_protection > doubleclick_ival {
                    println!("--");
                    thr.exti9_5.disable_int();
                    debounce_protection = debounce_ival;
                } else {
                    doubleclick_protection = 0;
                    println!("++");
                }
            }
        }
    }
    Event::Push
}
