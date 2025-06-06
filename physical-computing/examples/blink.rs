#![no_std]
#![no_main]
use cortex_m::delay::Delay;
use embedded_hal::digital::OutputPin;
use panic_halt as _;
use seeeduino_xiao_rp2040::{
    entry,
    hal::{self, pac, prelude::*, Sio},
    Pins, XOSC_CRYSTAL_FREQ,
};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut led_blue_pin = pins.led_blue.into_push_pull_output();
    let mut led_green_pin = pins.led_green.into_push_pull_output();
    let mut led_red_pin = pins.led_red.into_push_pull_output();

    led_green_pin.set_high().unwrap();
    led_red_pin.set_high().unwrap();
    loop {
        led_blue_pin.set_low().unwrap();

        delay.delay_ms(1000);

        led_blue_pin.set_high().unwrap();

        delay.delay_ms(500);
    }
}
