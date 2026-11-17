

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

#[derive(Clone, Copy, PartialEq)]
enum OperatingMode {
    LowPower,
    Balanced,
    HighPerformance,
}

impl OperatingMode {
    fn as_str(&self) -> &'static str {
        match self {
            Self::LowPower => "LOW_POWER",
            Self::Balanced => "BALANCED",
            Self::HighPerformance => "HIGH_PERFORMANCE",
        }
    }
}

fn select_mode(workload: u8) -> OperatingMode {
    match workload {
        0..=29 => OperatingMode::LowPower,
        30..=69 => OperatingMode::Balanced,
        _ => OperatingMode::HighPerformance,
    }
}

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default()
        .with_cpu_clock(CpuClock::max());

    let peripherals = esp_hal::init(config);

    let mut usb_serial =
        UsbSerialJtag::new(peripherals.USB_DEVICE);

    let mut state = ControlState::Init;
    let mut mode = OperatingMode::Balanced;
    let mut cycles: u32 = 0;
    let mut workload: u8 = 0;

    loop {
        state = match state {
            ControlState::Init => {
                writeln!(usb_serial, "INIT: Control system starting").ok();
                ControlState::Idle
            }

            ControlState::Idle => {
                let start = Instant::now();

                while start.elapsed() < Duration::from_secs(1) {}

                ControlState::Monitor
            }

            ControlState::Monitor => {
                cycles = cycles.wrapping_add(1);

                // Synthetic workload for policy testing.
                workload = (cycles % 101) as u8;

                let selected_mode = select_mode(workload);

                if selected_mode != mode {
                    writeln!(
                        usb_serial,
                        "MODE CHANGE: {} -> {}",
                        mode.as_str(),
                        selected_mode.as_str()
                    ).ok();

                    mode = selected_mode;
                }

                ControlState::Report
            }

            ControlState::Report => {
                writeln!(
                    usb_serial,
                    "REPORT: cycle={} workload={}% mode={}",
                    cycles,
                    workload,
                    mode.as_str()
                ).ok();

                ControlState::Idle
            }
        };
    }
}

