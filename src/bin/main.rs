

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

#[derive(Clone, Copy)]
enum ControlState {
    Init,
    Idle,
    Monitor,
    Report,
}

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default()
        .with_cpu_clock(CpuClock::max());

    let peripherals = esp_hal::init(config);

    let mut usb_serial =
        UsbSerialJtag::new(peripherals.USB_DEVICE);

    let mut state = ControlState::Init;
    let mut cycles: u32 = 0;

    loop {
        state = match state {
            ControlState::Init => {
                writeln!(usb_serial, "INIT: Control system starting").ok();
                ControlState::Idle
            }

            ControlState::Idle => {
                writeln!(usb_serial, "IDLE: Waiting for control interval").ok();

                let start = Instant::now();
                while start.elapsed() < Duration::from_secs(1) {}

                ControlState::Monitor
            }

            ControlState::Monitor => {
                cycles = cycles.wrapping_add(1);

                writeln!(
                    usb_serial,
                    "MONITOR: Cycle {}",
                    cycles
                ).ok();

                ControlState::Report
            }

            ControlState::Report => {
                writeln!(
                    usb_serial,
                    "REPORT: Completed cycle {}",
                    cycles
                ).ok();

                ControlState::Idle
            }
        };
    }
}

