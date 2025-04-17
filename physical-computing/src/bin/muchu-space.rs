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
        pins.rx.into_function(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut neopixel_power = pins.neopixel_power.into_push_pull_output();
    neopixel_power.set_high().unwrap();

    let mut trig_pin = pins.sda.into_push_pull_output();
    let mut echo_pin = pins.scl.into_floating_input();

    let mut hue = 0.0;
    let mut prev_cm = 0.0;

    loop {
        hue += 1.0;
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

        let current_cm = (duration as f32) * 0.36 / 2.0;
        let cm = if prev_cm < current_cm {
            let diff = current_cm - prev_cm;
            prev_cm + diff / 70.0
        } else {
            let diff = prev_cm - current_cm;
            prev_cm - diff / 70.0
        };
        prev_cm = cm;

        let lightness = if cm <= 150.0 {
            ((150.0 - cm) / 150.0 * 50.0) as f64
        } else {
            1.0
        };
        let rgb1 = hsl2rgb(hue, 50.0, lightness);
        let rgb2 = hsl2rgb(hue + 120.0, 50.0, lightness);
        let rgb3 = hsl2rgb(hue + 240.0, 50.0, lightness);
        ws.write([rgb1, rgb2, rgb3].iter().copied()).unwrap();

        delay.delay_ms(5);
    }
}

pub fn abs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

fn hsl2rgb(h: f64, s: f64, l: f64) -> RGB8 {
    let h = h % 360.0;
    let s = s / 100.0;
    let l = l / 100.0;

    let c = (1.0 - abs(2.0 * l - 1.0)) * s;
    let x = c * (1.0 - abs((h / 60.0) % 2.0 - 1.0));
    let m = l - c / 2.0;

    let (r, g, b) = if (0.0..60.0).contains(&h) {
        (c, x, 0.0)
    } else if (60.0..120.0).contains(&h) {
        (x, c, 0.0)
    } else if (120.0..180.0).contains(&h) {
        (0.0, c, x)
    } else if (180.0..240.0).contains(&h) {
        (0.0, x, c)
    } else if (240.0..300.0).contains(&h) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r + m) * 255.0) as u8;
    let g = ((g + m) * 255.0) as u8;
    let b = ((b + m) * 255.0) as u8;

    RGB8::new(r, g, b)
}
