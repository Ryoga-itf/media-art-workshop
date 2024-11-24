#![no_std]
#![no_main]
use embedded_hal::pwm::SetDutyCycle;
use panic_halt as _;
use seeeduino_xiao_rp2040::entry;
use seeeduino_xiao_rp2040::hal;
use seeeduino_xiao_rp2040::hal::pac;
use seeeduino_xiao_rp2040::hal::prelude::*;

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

    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);
    let pwm = &mut pwm_slices.pwm1;
    pwm.enable();
    // Set the PWM frequency to 50Hz
    pwm.set_top(24999);
    pwm.set_div_int(100);
    pwm.set_div_frac(0);

    let channel = &mut pwm.channel_b;
    channel.output_to(pins.mosi);

    loop {
        channel.set_duty_cycle(1250).unwrap();
        delay.delay_ms(5000);
        channel.set_duty_cycle(1875).unwrap();
        delay.delay_ms(5000);
        channel.set_duty_cycle(2500).unwrap();
        delay.delay_ms(5000);
    }
}
