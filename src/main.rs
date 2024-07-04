// #![deny(warnings)]
#![no_main]
#![no_std]

// use cortex_m_rt::entry;
use stm32h7xx_hal::{pac, prelude::*};

// release profile
// #[cfg(not(debug_assertions))]
extern crate panic_halt;

// use core::panic::PanicInfo;

/*
macro_rules! example_power {
  ($pwr:ident) => {{
      cfg_if::cfg_if! {
        if #[cfg(all(feature = "smps", feature = "example-smps"))] {
          $pwr.smps()
        }
        else if #[cfg(all(feature = "smps", feature = "example-ldo"))] {
          $pwr.ldo()
        }
        else {
          $pwr
        }
      }
  }};
}
*/

#[cortex_m_rt::entry]
// #[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg =  pwr.freeze();
                                      // example_power!(pwr).freeze();

    // Constrain and Freeze clock

    let rcc = dp.RCC.constrain();
    let ccdr = rcc
                    .sys_ck(100.MHz())
                    .freeze(pwrcfg, &dp.SYSCFG);

    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // Configure PE3 as output.
    let mut led = gpioe.pe3.into_push_pull_output();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);
    
    loop {
        led.set_high();
        delay.delay_ms(250_u16);

        led.set_low();
        delay.delay_ms(250_u16);
    }
}

/*
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // let mut host_stderr = HStderr::new();

    // logs "panicked at '$reason', src/main.rs:27:4" to the host stderr
    // writeln!(host_stderr, "{}", info).ok();

    loop {}
}
*/
