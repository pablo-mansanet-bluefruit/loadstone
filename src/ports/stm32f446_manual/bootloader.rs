//! Concrete bootloader construction and flash bank layout for stm32f446
//!
//! This is a manual port, meant to showcase a build that does not result
//! from configuration in loadstone_front.

use crate::devices::image;
use crate::{devices::bootloader::Bootloader, error};
use crate::error::Error;
use blue_hal::drivers::stm32f4;
use blue_hal::drivers::stm32f4::rcc::Clocks;
use blue_hal::drivers::stm32f4::serial::UsartExt;
use blue_hal::hal::null::{NullError, NullFlash};
use blue_hal::hal::time::Bps;
use blue_hal::stm32pac::USART1;
use super::pin_configuration::GpioExt;
use super::pin_configuration::*;

use blue_hal::drivers::stm32f4::{flash, serial, systick::SysTick};
use blue_hal::{KB, stm32pac};

#[cfg(feature="ecdsa-verify")]
use crate::devices::image::EcdsaImageReader as ImageReader;
#[cfg(not(feature="ecdsa-verify"))]
use crate::devices::image::CrcImageReader as ImageReader;

use super::update_signal::UpdateSignal;
type SerialPins = (Pb6<AF7>, Pb7<AF7>);
type Serial = stm32f4::serial::Serial<USART1, SerialPins>;

type ExternalFlash = NullFlash;

impl Default for Bootloader<ExternalFlash, flash::McuFlash, Serial, SysTick, ImageReader, UpdateSignal> {
    fn default() -> Self { Self::new() }
}

pub const NUMBER_OF_MCU_BANKS: usize = 1usize;
pub static MCU_BANKS: [image::Bank<flash::Address>; NUMBER_OF_MCU_BANKS] = [image::Bank {
    index: 1u8,
    bootable: true,
    location: flash::Address(0x0801_0000),
    size: KB!(448),
    is_golden: false,
}];

impl Bootloader<ExternalFlash, flash::McuFlash, Serial, SysTick, ImageReader, UpdateSignal> {
    pub fn new() -> Self {
        let mut peripherals = stm32pac::Peripherals::take().unwrap();
        let mcu_flash = flash::McuFlash::new(peripherals.FLASH).unwrap();

        let gpiob = peripherals.GPIOB.split(&mut peripherals.RCC);
        let clocks = Clocks::hardcoded(peripherals.RCC);
        let pins = (gpiob.pb6, gpiob.pb7);

        let config = serial::config::Config::default().baudrate(Bps(115_200));
        let serial = peripherals.USART1.constrain(pins, config, clocks).unwrap();

        Self {
            mcu_flash,
            mcu_banks: &MCU_BANKS,
            external_flash: None,
            external_banks: &[],
            serial: Some(serial),
            boot_metrics: Default::default(),
            start_time: None,
            recovery_enabled: true,
            update_signal: None,
            greeting: "Loadstone f446 Manual Port",
            _marker: Default::default(),
        }
    }
}

impl error::Convertible for flash::Error {
    fn into(self) -> Error {
        match self {
            flash::Error::MemoryNotReachable => Error::DriverError("[MCU Flash] Memory not reachable"),
            flash::Error::MisalignedAccess => Error::DriverError("[MCU Flash] Misaligned memory access"),
        }
    }
}

impl error::Convertible for NullError {
    fn into(self) -> Error { panic!("This error should never happen!") }
}

impl error::Convertible for serial::Error {
    fn into(self) -> Error {
        match self {
            serial::Error::Framing => Error::DriverError("[Serial] Framing error"),
            serial::Error::Noise => Error::DriverError("[Serial] Noise error"),
            serial::Error::Overrun => Error::DriverError("[Serial] Overrun error"),
            serial::Error::Parity => Error::DriverError("[Serial] Parity error"),
            serial::Error::Timeout => Error::DriverError("[Serial] Timeout error"),
            _ => Error::DriverError("[Serial] Unexpected serial error"),
        }
    }
}
