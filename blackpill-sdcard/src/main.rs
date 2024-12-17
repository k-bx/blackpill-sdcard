#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use cortex_m_rt::entry;
// use defmt::{debug, error, info, trace, warn, println};
// use defmt_semihosting as _;
use cortex_m_semihosting::hprintln;
use embedded_hal_nb::spi::MODE_0;
use embedded_sdmmc::{SdCard, VolumeManager};
use stm32f4xx_hal::{pac, prelude::*, rcc};

pub mod time;

// comment for defmt

macro_rules! trace {
    ($($arg:tt)*) => {
        hprintln!($($arg)*);
    };
}
macro_rules! debug {
    ($($arg:tt)*) => {
        hprintln!($($arg)*);
    };
}
macro_rules! info {
    ($($arg:tt)*) => {
        hprintln!($($arg)*);
    };
}
macro_rules! warn {
    ($($arg:tt)*) => {
        hprintln!($($arg)*);
    };
}
macro_rules! error {
    ($($arg:tt)*) => {
        hprintln!($($arg)*);
    };
}

#[entry]
fn main() -> ! {
    // println!("STARTING [println]");
    error!("STARTING [error]");
    warn!("STARTING [warn]");
    info!("STARTING [info]");
    debug!("STARTING [debug]");
    trace!("STARTING [trace]");

    let dp: pac::Peripherals = pac::Peripherals::take().unwrap();
    let _cp: cortex_m::peripheral::Peripherals = cortex_m::peripheral::Peripherals::take().unwrap();

    let rcc: rcc::Rcc = dp.RCC.constrain();
    let clocks: rcc::Clocks = rcc
        .cfgr
        .use_hse(25.MHz())
        .sysclk(100.MHz())
        .hclk(25.MHz())
        .freeze();
    let delay = dp.TIM5.delay_us(&clocks);
    let delay2 = dp.TIM2.delay_us(&clocks);

    let gpioa = dp.GPIOA.split();

    debug!("CREATNIG SPI1");
    let sd_spi = dp.SPI1.spi(
        (
            gpioa.pa5.into_alternate(),
            gpioa.pa6.into_alternate(),
            gpioa.pa7.into_alternate(),
        ),
        MODE_0,
        400.kHz(),
        &clocks,
    );
    let sd_cs = gpioa.pa4.into_push_pull_output();
    let spi = embedded_hal_bus::spi::ExclusiveDevice::new(sd_spi, sd_cs, delay).unwrap();
    let sdcard = SdCard::new(spi, delay2);
    let mut volume_mgr = VolumeManager::new(sdcard, time::ClockData::default());

    debug!("READING SD CARD SIZE");
    match volume_mgr.device().num_bytes() {
        Ok(size) => {
            info!("card size is {} bytes", size);
        }
        Err(e) => {
            // error!("Error retrieving card size: {}", defmt::Debug2Format(&e));
            error!("Error retrieving card size: {:?}", e);
        }
    }

    debug!("GETTING CARD TYPE");
    let device = volume_mgr.device();
    let card_type = device.get_card_type();
    debug!("Got card type: {:?}", card_type);

    loop {
        // your code goes here
    }
}
