#![no_std]
#![no_main]

use embedded_hal::digital::{InputPin, OutputPin};
use panic_halt as _;
use seeeduino_xiao_rp2040::{
    entry,
    hal::{self, pac, prelude::*, Timer},
};
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_pio::Ws2812;

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

    let mut trig_pin = pins.tx.into_push_pull_output();
    let mut echo_pin = pins.rx.into_floating_input();

    loop {
        trig_pin.set_low().unwrap();
        delay.delay_us(5);
        trig_pin.set_high().unwrap();
        delay.delay_us(10);
        trig_pin.set_low().unwrap();

        let mut duration = 0;
        while echo_pin.is_low().unwrap() {}
        while echo_pin.is_high().unwrap() {
            duration += 1;
            delay.delay_us(1);
        }

        let cm = (duration as f32) * 0.35 / 2.0;

        let rgb = if cm <= 100.0 {
            let blightness = ((100.0 - cm) / 100.0 * 255.0) as u8;
            RGB8::new(blightness, blightness, blightness)
        } else {
            RGB8::new(0, 0, 0)
        };
        ws.write([rgb].iter().copied()).unwrap();

        delay.delay_ms(250);
    }
}
