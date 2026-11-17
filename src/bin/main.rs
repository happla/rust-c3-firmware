#![no_std]
#![no_main]

use core::fmt::Write;

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_hal::usb::usb_serial_jtag::UsbSerialJtag;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default()
        .with_cpu_clock(CpuClock::max());

    let peripherals = esp_hal::init(config);

    let mut usb_serial =
        UsbSerialJtag::new(peripherals.USB_DEVICE);

    let mut counter: u32 = 0;

    loop {
        writeln!(
            usb_serial,
            "Rust ESP32-C3 heartbeat: {}",
            counter
        )
        .ok();

        counter += 1;

        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(1) {}
    }
}

