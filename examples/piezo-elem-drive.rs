#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use embedded_hal::pwm::SetDutyCycle;
use panic_halt as _;
use seeeduino_xiao_rp2040::{
    entry,
    hal::{clocks, pac, prelude::*, pwm::Slices, Sio, Watchdog},
    Pins,
};

const PWM_DIV: u8 = 100;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = clocks::init_clocks_and_plls(
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

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let mut pwm_slices = Slices::new(pac.PWM, &mut pac.RESETS);
    let pwm = &mut pwm_slices.pwm1;
    pwm.set_ph_correct();

    let buzzer_pin = pins.mosi.into_push_pull_output();

    pwm.enable();
    pwm.channel_b.output_to(buzzer_pin);
    pwm.set_div_int(PWM_DIV);

    fn calc_note(freq: f32) -> u16 {
        let system_clock = 125_000_000 as f32;
        (system_clock / PWM_DIV as f32 / freq) as u16
    }

    let tones = [
        523.0,  // C
        587.0,  // D
        659.0,  // E
        698.0,  // F
        783.0,  // G
        879.0,  // A
        987.0,  // B
        1046.0, // C
    ]
    .map(calc_note);

    loop {
        for &top in &tones {
            pwm.channel_b.set_duty_cycle(top / 2).unwrap();
            pwm.set_top(top);
            delay.delay_ms(500);
            pwm.channel_b.set_duty_cycle(0).unwrap();
            delay.delay_ms(500);
        }

        for &top in tones.iter().rev() {
            pwm.channel_b.set_duty_cycle(top / 2).unwrap();
            pwm.set_top(top);
            delay.delay_ms(500);
            pwm.channel_b.set_duty_cycle(0).unwrap();
            delay.delay_ms(500);
        }
    }
}
