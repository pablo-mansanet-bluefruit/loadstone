#[allow(unused_imports)]
use blue_hal::stm32pac::{self, USART1, USART2, USART6};
use blue_hal::{
    alternate_functions,
    drivers::stm32f4::{
        gpio::*,
        serial::{RxPin, TxPin},
    },
    enable_gpio, enable_qspi, enable_serial, enable_spi, gpio, gpio_inner, pin_rows,
};
pub type UsartPins = ();
pub type Serial = blue_hal::hal::null::NullSerial;
pub type ExternalFlash = blue_hal::hal::null::NullFlash;
pub type QspiPins = ();
enable_gpio!();
gpio ! (a , [(0 , Input < Floating >) , (1 , Input < Floating >) ,]) ;
gpio!(b, []);
gpio!(c, []);
gpio!(d, []);
gpio!(e, []);
gpio!(f, []);
gpio!(g, []);
gpio!(h, []);
#[allow(unused)]
pub fn pins(
    gpioa: stm32pac::GPIOA,
    gpiob: stm32pac::GPIOB,
    gpioc: stm32pac::GPIOC,
    gpiod: stm32pac::GPIOD,
    gpioe: stm32pac::GPIOE,
    gpiof: stm32pac::GPIOF,
    gpiog: stm32pac::GPIOG,
    gpioh: stm32pac::GPIOH,
    rcc: &mut stm32pac::RCC,
) -> (UsartPins, QspiPins) {
    let gpioa = gpioa.split(rcc);
    let gpiob = gpiob.split(rcc);
    let gpioc = gpioc.split(rcc);
    let gpiod = gpiod.split(rcc);
    let gpioe = gpioe.split(rcc);
    let gpiof = gpiof.split(rcc);
    let gpiog = gpiog.split(rcc);
    let gpioh = gpioh.split(rcc);
    ((), ())
}
