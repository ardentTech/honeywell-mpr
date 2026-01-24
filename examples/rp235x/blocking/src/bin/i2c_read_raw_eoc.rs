//! This example shows how to read raw sensor data using the rp235x-hal. Instead of waiting, an ISR
//! runs when the EOC pin goes high, which indicates that measurement data is ready.
#![no_std]
#![no_main]

use core::cell::RefCell;
use critical_section::Mutex;
use defmt::{error, info};
use defmt_rtt as _;
use embedded_hal::delay::DelayNs;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp235x_hal as hal;

// Some things we need
use hal::fugit::RateExtU32;
use hal::gpio::{FunctionI2C, Pin};
use rp235x_hal::gpio::bank0::Gpio13;
use rp235x_hal::gpio::{FunctionSioInput, PullDown};
use rp235x_hal::gpio::Interrupt::EdgeHigh;
use honeywell_mpr::{Mpr, MprConfig, TransferFunction};

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

/// External high-speed crystal on the Raspberry Pi Pico 2 board is 12 MHz.
/// Adjust if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

type EocPin = Pin<Gpio13, FunctionSioInput, PullDown>;
static EOC_PIN: Mutex<RefCell<Option<EocPin>>> = Mutex::new(RefCell::new(None));

/// Entry point to our bare-metal application.
///
/// The `#[hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the rp235x peripherals, then performs a single I²C
/// write to a fixed address.
#[hal::entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
        .unwrap();

    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let eoc: EocPin = pins.gpio13.reconfigure();
    eoc.set_interrupt_enabled(EdgeHigh, true);
    critical_section::with(|cs| {
        EOC_PIN.borrow(cs).replace(Some(eoc));
    });

    // Configure two pins as being I²C, not GPIO
    let sda_pin: Pin<_, FunctionI2C, _> = pins.gpio14.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, _> = pins.gpio15.reconfigure();

    // Create the I²C drive, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let bus = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin, // Try `not_an_scl_pin` here
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    let config = MprConfig::new(0, 25, TransferFunction::C);
    let mut sensor = Mpr::new_i2c(bus, 0x18, config).unwrap();

    // interrupt_unmask is not on rp235x-hal 0.3.0
    unsafe {
        cortex_m::peripheral::NVIC::unmask(hal::pac::Interrupt::IO_IRQ_BANK0)
    }
    unsafe {
        hal::arch::interrupt_enable();
    }

    loop {
        sensor.exit_standby().unwrap();
        hal::arch::wfi();
        match sensor.read_raw() {
            Ok(raw_data) => info!("raw data: {}", raw_data),
            Err(_) => error!("read_raw failed :(")
        }
        timer.delay_ms(3_000);
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
fn IO_IRQ_BANK0() {
    critical_section::with(|cs| {
        let mut maybe_eoc = EOC_PIN.borrow_ref_mut(cs);
        if let Some(eoc) = maybe_eoc.as_mut() {
            if eoc.interrupt_status(EdgeHigh) {
                eoc.clear_interrupt(EdgeHigh);
            }
        }
    });
}

/// Program metadata for `picotool info`
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
    hal::binary_info::rp_cargo_bin_name!(),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(c"I²C Example"),
    hal::binary_info::rp_cargo_homepage_url!(),
    hal::binary_info::rp_program_build_attribute!(),
];