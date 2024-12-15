#![no_std]
#![no_main]
use cortex_m::asm::delay;
use embedded_hal::digital::OutputPin;
use panic_halt as _;
use seeeduino_xiao_rp2040::entry;
use seeeduino_xiao_rp2040::hal;
use seeeduino_xiao_rp2040::hal::pac;
use seeeduino_xiao_rp2040::hal::prelude::*;
use seeeduino_xiao_rp2040::hal::usb::UsbBus;
use seeeduino_xiao_rp2040::hal::Timer;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::UsbDeviceBuilder;
use usb_device::device::UsbVidPid;
use usbd_serial::SerialPort;
use usbd_serial::USB_CLASS_CDC;

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

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let sio = hal::Sio::new(pac.SIO);
    let pins = seeeduino_xiao_rp2040::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

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

    let mut last_time = timer.get_counter();
    loop {
        if usb_dev.poll(&mut [&mut serial]) {
            let current_time = timer.get_counter();
            if current_time.ticks() > last_time.ticks() + 1_000_000 {
                last_time = current_time;
                serial.write(b"hello!\r\n").unwrap();
            }
        }
    }
}
