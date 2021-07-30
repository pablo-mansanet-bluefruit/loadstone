#[allow(unused_imports)]
use blue_hal::stm32pac::{self, USART1, USART2, USART6};
use blue_hal::{
    alternate_functions,
    drivers::stm32f4::gpio::*,
    drivers::stm32f4::serial::*,
    enable_gpio, enable_qspi, enable_serial, enable_spi, gpio, gpio_inner, pin_rows,
};

enable_gpio!();
gpio ! (a, []);
gpio!(b, [(6, AF7 as TxPin<USART1>), (7, AF7 as RxPin<USART1>),]);
gpio!(c, []);
gpio!(d, []);
gpio!(e, []);
gpio!(f, []);
gpio!(g, []);
gpio!(h, []);
