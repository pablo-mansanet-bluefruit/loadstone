use blue_hal::{alternate_functions, gpio, gpio_inner, pin_rows};
use blue_hal::paste;
use blue_hal::drivers::stm32f4::gpio::{self, *};
use blue_hal::hal::gpio::{*, InputPin};
use blue_hal::drivers::stm32f4::serial::{TxPin, RxPin};
use blue_hal::stm32pac::USART6;
use blue_hal::drivers::stm32f4::qspi::{
    ClkPin as QspiClk,
    Bk1CsPin as QspiChipSelect,
    Bk1Io0Pin as QspiOutput,
    Bk1Io1Pin as QspiInput,
    Bk1Io2Pin as QspiSecondaryOutput,
    Bk1Io3Pin as QspiSecondaryInput,
};
alternate_functions!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,);
pin_rows!(a, b, c, d, e, f, g, h, i, j, k,);

gpio!(a, [
    (0, Input<Floating>), // Boot mode
    (1, Input<Floating>),
]);
gpio!(e, [(1, Output<PushPull>),]); // LED
gpio!(b, [(2, AF9 as QspiClk),]);
gpio!(f, [
    (6, AF9 as QspiSecondaryInput),
    (7, AF9 as QspiSecondaryOutput),
    (8, AF10 as QspiOutput),
    (9, AF10 as QspiInput),
]);
gpio!(g, [
    (6, AF10 as QspiChipSelect),
    (14, AF8 as TxPin<USART6>),
    (9, AF8 as RxPin<USART6>),
]);
