#![no_std]
#![no_main]
use panic_halt as _;
use seeeduino_xiao_rp2040::{
    entry,
    hal::{self, pac, usb::UsbBus, Timer},
};
use usb_device::{bus::UsbBusAllocator, device::UsbDeviceBuilder, device::UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

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
