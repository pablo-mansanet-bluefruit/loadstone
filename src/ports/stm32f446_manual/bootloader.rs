//! Concrete bootloader construction and flash bank layout for stm32f446
//!
//! This is a manual port, meant to showcase a build that does not result
//! from configuration in loadstone_front.
use crate::devices::update_signal::ReadUpdateSignal;
use crate::devices;
use crate::{devices::bootloader::Bootloader, error};
use crate::error::Error;
use blue_hal::hal::null::{NullError, NullFlash, NullSerial};

use blue_hal::drivers::stm32f4::{flash, serial, systick::SysTick};

#[cfg(feature="ecdsa-verify")]
use crate::devices::image::EcdsaImageReader as ImageReader;
#[cfg(not(feature="ecdsa-verify"))]
use crate::devices::image::CrcImageReader as ImageReader;

type ExternalFlash = NullFlash;
type Serial = NullSerial;
pub struct UpdateSignal;

impl ReadUpdateSignal for UpdateSignal {
    fn read_update_plan(&self) -> devices::update_signal::UpdatePlan {
        todo!()
    }
}

impl Default for Bootloader<ExternalFlash, flash::McuFlash, Serial, SysTick, ImageReader, UpdateSignal> {
    fn default() -> Self { Self::new() }
}

impl Bootloader<ExternalFlash, flash::McuFlash, Serial, SysTick, ImageReader, UpdateSignal> {
    pub fn new() -> Self {
        todo!();
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
