#![no_std]
#![no_main]
use embedded_hal::digital::OutputPin;
use hal::pio::PIOExt;
use hal::Timer;
use panic_halt as _;
use seeeduino_xiao_rp2040::entry;
use seeeduino_xiao_rp2040::hal;
use seeeduino_xiao_rp2040::hal::pac;
use seeeduino_xiao_rp2040::hal::prelude::*;
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_pio::Ws2812;
const RED: RGB8 = RGB8::new(255, 0, 0);
const GREEN: RGB8 = RGB8::new(0, 255, 0);
const BLUE: RGB8 = RGB8::new(0, 0, 255);
const WHITE: RGB8 = RGB8::new(255, 255, 255);

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        seeeduino_xiao_rp2040::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pins = seeeduino_xiao_rp2040::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut ws = Ws2812::new(
        pins.neopixel_data.into_function(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut neopixel_power = pins.neopixel_power.into_push_pull_output();
    neopixel_power.set_high().unwrap();

    let mut led_blue_pin = pins.led_blue.into_push_pull_output();
    let mut led_green_pin = pins.led_green.into_push_pull_output();
    let mut led_red_pin = pins.led_red.into_push_pull_output();

    loop {
        // Set USER LED to blue
        led_blue_pin.set_low().unwrap();
        led_red_pin.set_high().unwrap();
        led_green_pin.set_high().unwrap();
        // Set RGB LED to blue
        ws.write([BLUE].iter().copied()).unwrap();
        delay.delay_ms(500);
        // Set USER LED to red
        led_blue_pin.set_high().unwrap();
        led_red_pin.set_low().unwrap();
        led_green_pin.set_high().unwrap();
        // Set RGB LED to red
        ws.write([RED].iter().copied()).unwrap();
        delay.delay_ms(500);
        // Set USER LED to green
        led_blue_pin.set_high().unwrap();
        led_red_pin.set_high().unwrap();
        led_green_pin.set_low().unwrap();
        // Set RGB LED to red
        ws.write([GREEN].iter().copied()).unwrap();
        delay.delay_ms(500);
        // Set USER LED to white
        led_blue_pin.set_low().unwrap();
        led_red_pin.set_low().unwrap();
        led_green_pin.set_low().unwrap();
        // Set RGB LED to white
        ws.write([WHITE].iter().copied()).unwrap();
        delay.delay_ms(500);
    }
}
