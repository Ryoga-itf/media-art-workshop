#![no_std]
#![no_main]
use cortex_m::delay::Delay;
use embedded_hal::digital::{InputPin, OutputPin};
use panic_halt as _;
use seeeduino_xiao_rp2040::{
    entry,
    hal::{self, pac, usb::UsbBus, Clock, Timer},
    Pins,
};
use usb_device::{bus::UsbBusAllocator, device::UsbDeviceBuilder, device::UsbVidPid};
use usbd_serial::{embedded_io::Write, SerialPort, USB_CLASS_CDC};

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
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let usb_bus = UsbBusAllocator::new(UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .device_class(USB_CLASS_CDC)
        .build();

    let mut trig_pin = pins.tx.into_push_pull_output();
    let mut echo_pin = pins.rx.into_floating_input();

    let mut last_time = timer.get_counter();
    loop {
        if usb_dev.poll(&mut [&mut serial]) {
            let current_time = timer.get_counter();
            if current_time.ticks() > last_time.ticks() + 1_000_000 {
                last_time = current_time;

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

                let cm = (duration as f32) * 0.36 / 2.0;

                write!(serial, "Value: {}\r\n", cm).unwrap();
            }
        }
    }
}
