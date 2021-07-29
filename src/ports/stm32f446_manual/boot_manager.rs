//! Concrete bootloader construction and flash bank layout for stm32f446
//!
//! This is a manual port, meant to showcase a build that does not result
//! from configuration in loadstone_front.

use crate::devices::cli::Cli;
use crate::devices::boot_manager::BootManager;
use blue_hal::drivers::stm32f4;
use blue_hal::drivers::stm32f4::rcc::Clocks;
use blue_hal::drivers::stm32f4::serial::UsartExt;
use blue_hal::hal::null::NullFlash;
use blue_hal::hal::time::Bps;
use blue_hal::stm32pac::USART1;
use super::bootloader::MCU_BANKS;
use super::pin_configuration::GpioExt;
use super::pin_configuration::*;

use blue_hal::drivers::stm32f4::{flash, serial};
use blue_hal::stm32pac;

#[cfg(feature="ecdsa-verify")]
use crate::devices::image::EcdsaImageReader as ImageReader;
#[cfg(not(feature="ecdsa-verify"))]
use crate::devices::image::CrcImageReader as ImageReader;

use super::update_signal::UpdateSignalWriter;
type SerialPins = (Pb6<AF7>, Pb7<AF7>);
type Serial = stm32f4::serial::Serial<USART1, SerialPins>;

type ExternalFlash = NullFlash;

impl Default for BootManager<flash::McuFlash, ExternalFlash , Serial, ImageReader, UpdateSignalWriter> {
    fn default() -> Self { Self::new() }
}

impl BootManager<flash::McuFlash, ExternalFlash, Serial, ImageReader, UpdateSignalWriter> {
    pub fn new() -> Self {
        let mut peripherals = stm32pac::Peripherals::take().unwrap();
        let mcu_flash = flash::McuFlash::new(peripherals.FLASH).unwrap();

        let gpiob = peripherals.GPIOB.split(&mut peripherals.RCC);
        let clocks = Clocks::hardcoded(peripherals.RCC);
        let pins = (gpiob.pb6, gpiob.pb7);

        let config = serial::config::Config::default().baudrate(Bps(115_200));
        let serial = peripherals.USART1.constrain(pins, config, clocks).unwrap();
        let cli = Cli::new(serial).unwrap();

        Self {
            mcu_flash,
            mcu_banks: &MCU_BANKS,
            external_flash: None,
            external_banks: &[],
            cli: Some(cli),
            boot_metrics: Default::default(),
            update_signal: None,
            greeting: Some("Loadstone f446 Manual Port Demo App"),
            _marker: Default::default(),
        }
    }
}
