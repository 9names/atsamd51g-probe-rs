#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use atsamd_hal as hal;
use cortex_m_rt::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::watchdog::{Watchdog, WatchdogTimeout};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    delay.delay_ms(400u16);

    let mut wdt = Watchdog::new(peripherals.WDT);
    wdt.start(WatchdogTimeout::Cycles256 as u8);

    loop {
        for _ in 0..4 {
            delay.delay_ms(200u8);
            wdt.feed();
        }
        info!("still alive!");
    }
}
