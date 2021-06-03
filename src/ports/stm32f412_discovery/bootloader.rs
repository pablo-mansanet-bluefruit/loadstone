//! Concrete bootloader construction and flash bank layout
//! for the [stm32f412 discovery](../../../../loadstone/hardware/discovery.pdf).
use crate::{devices::bootloader::Bootloader, error};
use crate::error::Error;
use blue_hal::{drivers::{micron::n25q128a_flash, stm32f4::{flash, qspi::{self, mode}, rcc::Clocks, serial, systick::SysTick}}, hal::time, stm32pac};
use crate::autogenerated::pin_configuration::{self, *};
use crate::autogenerated::memory_map::EXTERNAL_BANKS;
use crate::autogenerated::memory_map::MCU_BANKS;

#[cfg(feature = "serial")]
use crate::autogenerated::devices;

impl Default for Bootloader<ExternalFlash, flash::McuFlash, Serial, SysTick> {
    fn default() -> Self { Self::new() }
}

impl Bootloader<ExternalFlash, flash::McuFlash, Serial, SysTick> {
    pub fn new() -> Self {
        let mut peripherals = stm32pac::Peripherals::take().unwrap();
        let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
        let mcu_flash = flash::McuFlash::new(peripherals.FLASH).unwrap();

        let (serial_pins, qspi_pins) = pin_configuration::pins(
                peripherals.GPIOA,
                peripherals.GPIOB,
                peripherals.GPIOC,
                peripherals.GPIOD,
                peripherals.GPIOE,
                peripherals.GPIOF,
                peripherals.GPIOG,
                peripherals.GPIOH,
                &mut peripherals.RCC,
            );
        let clocks = Clocks::hardcoded(peripherals.RCC);
        SysTick::init(cortex_peripherals.SYST, clocks);
        SysTick::wait(time::Seconds(1)); // Gives time for the flash chip to stabilize after powerup
        let qspi_config = qspi::Config::<mode::Single>::default().with_flash_size(24).unwrap();
        let qspi = Qspi::from_config(peripherals.QUADSPI, qspi_pins, qspi_config).unwrap();
        let external_flash = ExternalFlash::with_timeout(qspi, time::Milliseconds(5000)).unwrap();

        #[cfg(feature = "serial")]
        let serial = Some(devices::construct_serial(serial_pins, clocks, peripherals.USART1, peripherals.USART2, peripherals.USART6));
        #[cfg(not(feature = "serial"))]
        let serial = None;

        #[cfg(feature = "boot-time-metrics")]
        let start_time = Some(SysTick::now());
        #[cfg(not(feature = "boot-time-metrics"))]
        let start_time = None;

        Bootloader {
            mcu_flash,
            external_banks: &EXTERNAL_BANKS,
            mcu_banks: &MCU_BANKS,
            external_flash: Some(external_flash),
            serial,
            boot_metrics: Default::default(),
            start_time,
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

impl error::Convertible for n25q128a_flash::Error {
    fn into(self) -> Error {
        match self {
            n25q128a_flash::Error::TimeOut => Error::DriverError("[External Flash] Operation timed out"),
            n25q128a_flash::Error::QspiError => Error::DriverError("[External Flash] Qspi error"),
            n25q128a_flash::Error::WrongManufacturerId => Error::DriverError("[External Flash] Wrong manufacturer ID"),
            n25q128a_flash::Error::MisalignedAccess => Error::DriverError("[External Flash] Misaligned memory access"),
            n25q128a_flash::Error::AddressOutOfRange => Error::DriverError("[External Flash] Address out of range"),
        }
    }
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
