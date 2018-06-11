use core::marker::PhantomData;
use core::convert::TryFrom;

use nb;
use bitrate::{Bps, Hertz};
use bit_field::BitField;
use hal::serial;
use mk20d7::{self, UART0, UART1, UART2};
use void::Void;

use mcg::MultipurposeClockGenerator;
use gpio::{
    Alternate, ALT2, ALT3,
    gpioa::{PTA1, PTA2},
    gpiob::{PTB16, PTB17},
    gpioc::{PTC3, PTC4},
    gpiod::{PTD2, PTD3, PTD6, PTD7},
    gpioe::{PTE0, PTE1},
};

/// Interrupt event
pub enum Event {
    /// New data has been received
    Rxne,

    /// New data can be sent
    Txe,
}

/// Serial error
#[derive(Debug)]
pub enum Error {
    /// Framing error
    Framing,

    /// Noise error
    Noise,

    /// RX buffer overrun
    Overrun,

    /// Parity check error
    Parity,

    #[doc(hidden)]
    _Extensible,
}

// FIXME these should be a "sealed" trait
/// TX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait TxPin<UART> {}

// FIXME these should be a "sealed" trait
/// RX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RxPin<UART> {}

// UART 0 PTA
unsafe impl RxPin<UART0> for PTA1<Alternate<ALT2>> {}
unsafe impl TxPin<UART0> for PTA2<Alternate<ALT2>> {}

// UART 0 PTB
unsafe impl RxPin<UART0> for PTB17<Alternate<ALT3>> {}
unsafe impl TxPin<UART0> for PTB16<Alternate<ALT3>> {}

// UART 0 PTD
unsafe impl RxPin<UART0> for PTD6<Alternate<ALT3>> {}
unsafe impl TxPin<UART0> for PTD7<Alternate<ALT3>> {}

// UART 1 PTC
unsafe impl RxPin<UART1> for PTC3<Alternate<ALT3>> {}
unsafe impl TxPin<UART1> for PTC4<Alternate<ALT3>> {}

// UART 1 PTE
unsafe impl RxPin<UART1> for PTE1<Alternate<ALT3>> {}
unsafe impl TxPin<UART1> for PTE0<Alternate<ALT3>> {}

// UART 2 PTD
unsafe impl RxPin<UART1> for PTD2<Alternate<ALT3>> {}
unsafe impl TxPin<UART1> for PTD3<Alternate<ALT3>> {}


/// Serial abstraction
pub struct Serial<UART, PINS> {
    uart: UART,
    pins: PINS,
}

/// Serial receiver
pub struct Rx<UART> {
    _uart: PhantomData<UART>,
}

/// Serial transmitter
pub struct Tx<UART> {
    _uart: PhantomData<UART>,
}

macro_rules! hal {
    ($(
        $UARTX:ident: ($uartX:ident),
    )+) => {
        $(
            impl<TX, RX> Serial<$UARTX, (TX, RX)> {
                // Reference 47.8.3 Initialization sequence (non ISO-7816)
                /// Configures a UART peripheral to provide serial communication
                pub fn $uartX(
                    uart: $UARTX,
                    pins: (TX, RX),
                    baud_rate: Bps<u32>,
                    mcg: &MultipurposeClockGenerator,
                ) -> Self
                where
                    TX: TxPin<$UARTX>,
                    RX: RxPin<$UARTX>,
                {
                    // Reference: 47.4.4 Baud rate generation
                    let clock: Hertz<u32> = mcg.get_pll_frequency().into();
                    let numerator = clock.0;
                    let denominator = baud_rate.0 * mcg.external_crystal_frequency.0;

                    let module_clock_divisor_main = numerator / denominator;

                    if module_clock_divisor_main >= 8192 {
                        panic!("Invalid UART clock divider: {}", module_clock_divisor_main);
                    }

                    let module_clock_divisor_gcd = gcd(numerator, denominator);
                    let module_clock_divisor_fine_adjustment_denominator = {
                        denominator / module_clock_divisor_gcd
                    };
                    let module_clock_divisor_fine_adjustment_numerator = {
                        (numerator / module_clock_divisor_gcd) - (module_clock_divisor_main * module_clock_divisor_fine_adjustment_denominator)
                    };
                    let module_clock_divisor_fine_adjustment = u8::try_from(
                        module_clock_divisor_fine_adjustment_numerator * 32 /
                        module_clock_divisor_fine_adjustment_denominator
                    ).unwrap();

                    // Reference: 47.3.11 UART Control Register 4 (UART_C4)
                    uart.c4.write(|w| unsafe { w.brfa().bits(module_clock_divisor_fine_adjustment) });

                    // Reference: 47.3.1 UART Baud Rate Registers: High (UART_BDH)
                    let module_clock_divisor_high = module_clock_divisor_main.get_bits(8..13) as u8;
                    uart.bdh.write(|w| unsafe { w.sbr().bits(module_clock_divisor_high) });

                    // Reference: 47.3.2 UART Baud Rate Registers: Low (UART_BDL)
                    let module_clock_divisor_low = module_clock_divisor_main.get_bits(0..8) as u8;
                    uart.bdl.write(|w| unsafe { w.sbr().bits(module_clock_divisor_low) });

                    // Reference: 47.3.4 UART Control Register 2 (UART_C2)
                    uart.c2.write(|w| {
                        w.re().set_bit();
                        w.te().set_bit()
                    });

                    Serial { uart, pins }
                }

                /// Splits the `Serial` abstraction into a transmitter and a receiver half
                pub fn split(self) -> (Tx<$UARTX>, Rx<$UARTX>) {
                    (Tx { _uart: PhantomData }, Rx { _uart: PhantomData })
                }

                /// Releases the UART peripheral and associated pins
                pub fn free(self) -> ($UARTX, (TX, RX)) {
                    (self.uart, self.pins)
                }
            }

            fn $uartX<'a>() -> &'a mk20d7::$uartX::RegisterBlock {
                unsafe { &(*$UARTX::ptr()) }
            }

            impl serial::Read<u8> for Rx<$UARTX> {
                type Error = Error;

                fn read(&mut self) -> nb::Result<u8, Error> {
                    let uart = $uartX();
                    let s1 = uart.s1.read();

                    if s1.pf().bit_is_set() {
                        return Err(nb::Error::Other(Error::Parity));
                    }

                    if s1.fe().bit_is_set() {
                        return Err(nb::Error::Other(Error::Framing));
                    }

                    if s1.nf().bit_is_set() {
                        return Err(nb::Error::Other(Error::Noise));
                    }

                    if s1.or().bit_is_set() {
                        return Err(nb::Error::Other(Error::Overrun));
                    }

                    if s1.rdrf().bit_is_clear() {
                        return Err(nb::Error::WouldBlock);
                    }

                    Ok(uart.d.read().rt().bits())
                }
            }

            impl serial::Write<u8> for Tx<$UARTX> {
                // The only possible errors during
                // transmission are: clear to send (which is disabled in this case) errors and
                // framing errors (which only occur in SmartCard mode); neither of these apply to
                // our hardware configuration
                type Error = Void;

                fn flush(&mut self) -> nb::Result<(), Void> {
                    if $uartX().s1.read().tc().bit_is_clear() {
                        return Err(nb::Error::WouldBlock);
                    }

                    Ok(())
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), Void> {
                    let uart = $uartX();

                    if uart.s1.read().tdre().bit_is_clear() {
                        return Err(nb::Error::WouldBlock);
                    }

                    uart.d.write(|w| unsafe { w.bits(byte) });

                    Ok(())
                }
            }
        )+
    }
}

hal! {
    UART0: (uart0),
    UART1: (uart1),
    UART2: (uart2),
}

// Euclid's GCD
fn gcd(numerator: u32, denominator: u32) -> u32 {
    let mut numerator = numerator;
    let mut denominator = denominator;
    while denominator != 0 {
        let temp = denominator;
        denominator = numerator % denominator;
        numerator = temp;
    }
    numerator
}
