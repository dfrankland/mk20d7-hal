use core::marker::PhantomData;
use mk20d7::sim::SCGC5;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, scgc5: &SCGC5) -> Self::Parts;
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;

/// Pulled down input (type state)
pub struct PullDown;

/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;

/// Open drain output (type state)
pub struct OpenDrain;

/// Alternate function 0 (type state, Analog)
pub struct ALT0;

/// Alternate function 1 (type state, GPIO)
pub struct ALT1;

/// Alternate function 2 (type state, chip specific)
pub struct ALT2;

/// Alternate function 3 (type state, chip specific)
pub struct ALT3;

/// Alternate function 4 (type state, chip specific)
pub struct ALT4;

/// Alternate function 5 (type state, chip specific)
pub struct ALT5;

/// Alternate function 6 (type state, chip specific)
pub struct ALT6;

/// Alternate function 7 (type state, chip specific / JTAG / NMI)
pub struct ALT7;

// Pin mux controller mode
enum PinMux {
    #[allow(dead_code)]
    ALT0,

    ALT1,

    #[allow(dead_code)]
    ALT2,

    #[allow(dead_code)]
    ALT3,

    #[allow(dead_code)]
    ALT4,

    #[allow(dead_code)]
    ALT5,

    #[allow(dead_code)]
    ALT6,

    #[allow(dead_code)]
    ALT7,
}

// Pin mode (when pin is in ALT1 gpio mode)
enum PinMode {
    Output,

    #[allow(dead_code)]
    Input,
}

macro_rules! gpio {
    ($PORTX:ident, $portx:ident, $PTX:ident, $ptx:ident, $gpiox:ident, $docport:expr, [ $($PTXi:ident: ($ptxi:ident, $i:expr, $MODE:ty, $docpin:expr),)+]) =>
    {
        #[doc = "General Purpose Input/Output Port "]
        #[doc = $docport]
        pub mod $gpiox {
            use core::marker::PhantomData;

            use hal::digital::{
                OutputPin, StatefulOutputPin, InputPin,
                toggleable,
            };

            use mk20d7::{sim::SCGC5, $PORTX, $PTX, $portx, $ptx};

            use super::{
                Floating, GpioExt, Input, Output,
                PushPull,
                PinMux, PinMode,
            };

            /// General Purpose Input/Output and Pin Control and Interrupts parts
            pub struct Parts {
                // Pin Control and Interrupts parts
                /// Digital Filter Clock Register
                pub dfcr: DFCR,

                /// Digital Filter Enable Register
                pub dfer: DFER,

                /// Digital Filter Width Register
                pub dfwr: DFWR,

                /// Global Pin Control High Register
                pub gpchr: GPCHR,

                /// Global Pin Control Low Register
                pub gpclr: GPCLR,

                /// Interrupt Status Flag Register
                pub isfr: ISFR,

                /// Pin Control Register n
                pub pcr: PCR,

                // General Purpose Input/Output parts
                /// Port Clear Output Register
                pub pcor: PCOR,

                /// Port Data Direction Register
                pub pddr: PDDR,

                /// Port Data Input Register
                pub pdir: PDIR,

                /// Port Data Output Register
                pub pdor: PDOR,

                /// Port Set Output Register
                pub psor: PSOR,

                /// Port Toggle Output Register
                pub ptor: PTOR,

                $(
                    #[doc = "General Purpose Input/Output Port "]
                    #[doc = $docport]
                    #[doc = " Pin "]
                    #[doc = $docpin]
                    pub $ptxi: $PTXi<$MODE>,
                )+
            }

            impl GpioExt for ($PTX, $PORTX) {
                type Parts = Parts;

                fn split(self, scgc5: &SCGC5) -> Self::Parts {
                    // Enable the GPIO module
                    // Reference: 10.2.3 Clock gating
                    scgc5.write(|w| w.$portx().set_bit());

                    Parts {
                        dfcr: DFCR { _0: () },
                        dfer: DFER { _0: () },
                        dfwr: DFWR { _0: () },
                        gpchr: GPCHR { _0: () },
                        gpclr: GPCLR { _0: () },
                        isfr: ISFR { _0: () },
                        pcr: PCR { _0: () },
                        pcor: PCOR { _0: () },
                        pddr: PDDR { _0: () },
                        pdir: PDIR { _0: () },
                        pdor: PDOR { _0: () },
                        psor: PSOR { _0: () },
                        ptor: PTOR { _0: () },
                        $(
                            $ptxi: $PTXi {_mode: PhantomData},
                        )+
                    }
                }
            }

            /// Digital Filter Clock Register
            pub struct DFCR {
                _0: (),
            }

            impl DFCR {
                #[allow(dead_code)]
                pub(crate) fn dfcr(&mut self) -> &$portx::DFCR {
                    unsafe { &(*$PORTX::ptr()).dfcr }
                }
            }

            /// Digital Filter Enable Register
            pub struct DFER {
                _0: (),
            }

            impl DFER {
                #[allow(dead_code)]
                pub(crate) fn dfer(&mut self) -> &$portx::DFER {
                    unsafe { &(*$PORTX::ptr()).dfer }
                }
            }

            /// Digital Filter Width Register
            pub struct DFWR {
                _0: (),
            }

            impl DFWR {
                #[allow(dead_code)]
                pub(crate) fn dfwr(&mut self) -> &$portx::DFWR {
                    unsafe { &(*$PORTX::ptr()).dfwr }
                }
            }

            /// Global Pin Control High Register
            pub struct GPCHR {
                _0: (),
            }

            /// Global Pin Control Low Register
            pub struct GPCLR {
                _0: (),
            }

            /// Interrupt Status Flag Register
            pub struct ISFR {
                _0: (),
            }

            impl ISFR {
                #[allow(dead_code)]
                pub(crate) fn isfr(&mut self) -> &$portx::ISFR {
                    unsafe { &(*$PORTX::ptr()).isfr }
                }
            }

            /// Pin Control Register n
            pub struct PCR {
                _0: (),
            }

            impl PCR {
                pub(crate) fn pcr(&mut self) -> &[$portx::PCR; 32] {
                    unsafe { &(*$PORTX::ptr()).pcr }
                }
            }

            /// Port Clear Output Register
            pub struct PCOR {
                _0: (),
            }

            impl PCOR {
                #[allow(dead_code)]
                pub(crate) fn pcor(&mut self) -> &$ptx::PCOR {
                    unsafe { &(*$PTX::ptr()).pcor }
                }
            }

            /// Port Data Direction Register
            pub struct PDDR {
                _0: (),
            }

            impl PDDR {
                pub(crate) fn pddr(&mut self) -> &$ptx::PDDR {
                    unsafe { &(*$PTX::ptr()).pddr }
                }
            }

            /// Port Data Input Register
            pub struct PDIR {
                _0: (),
            }

            impl PDIR {
                #[allow(dead_code)]
                pub(crate) fn pdir(&mut self) -> &$ptx::PDIR {
                    unsafe { &(*$PTX::ptr()).pdir }
                }
            }

            /// Port Data Output Register
            pub struct PDOR {
                _0: (),
            }

            impl PDOR {
                #[allow(dead_code)]
                pub(crate) fn pdor(&mut self) -> &$ptx::PDOR {
                    unsafe { &(*$PTX::ptr()).pdor }
                }
            }

            /// Port Set Output Register
            pub struct PSOR {
                _0: (),
            }

            impl PSOR {
                #[allow(dead_code)]
                pub(crate) fn psor(&mut self) -> &$ptx::PSOR {
                    unsafe { &(*$PTX::ptr()).psor }
                }
            }

            /// Port Toggle Output Register
            pub struct PTOR {
                _0: (),
            }

            impl PTOR {
                #[allow(dead_code)]
                pub(crate) fn ptor(&mut self) -> &$ptx::PTOR {
                    unsafe { &(*$PTX::ptr()).ptor }
                }
            }

            fn set_pin_mux(pin: usize, pcr: &mut PCR, pin_mux: PinMux) {
                let alt = match pin_mux {
                    PinMux::ALT0 => $portx::pcr::MUXW::_000,
                    PinMux::ALT1 => $portx::pcr::MUXW::_001,
                    PinMux::ALT2 => $portx::pcr::MUXW::_010,
                    PinMux::ALT3 => $portx::pcr::MUXW::_011,
                    PinMux::ALT4 => $portx::pcr::MUXW::_100,
                    PinMux::ALT5 => $portx::pcr::MUXW::_101,
                    PinMux::ALT6 => $portx::pcr::MUXW::_110,
                    PinMux::ALT7 => $portx::pcr::MUXW::_111,
                };

                pcr.pcr()[pin].write(|w| w.mux().variant(alt));
            }

            fn set_pin_mode(pin: u32, pddr: &mut PDDR, pin_mode: PinMode) {
                let bit = match pin_mode {
                    PinMode::Output => 1 << pin,
                    PinMode::Input => !(1 << pin),
                };
                pddr.pddr().modify(|r, w| unsafe { w.bits(r.bits() | bit) });
            }

            // This pin owns its section of the PDOR, PSOR, PCOR, PTOR, and PDIR registers, as well
            // as its PCR register
            // Reference: 11.14.1 Pin Control Register n (PORTx_PCRn)
            $(
                #[doc = "General Purpose Input/Output Port "]
                #[doc = $docport]
                #[doc = " Pin "]
                #[doc = $docpin]
                pub struct $PTXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PTXi<MODE> {
                    pub fn into_push_pull_output(self, pcr: &mut PCR, pddr: &mut PDDR) -> $PTXi<Output<PushPull>> {
                        set_pin_mux($i, pcr, PinMux::ALT1);
                        set_pin_mode($i, pddr, PinMode::Output);
                        $PTXi { _mode: PhantomData }
                    }
                }

                impl<MODE> StatefulOutputPin for $PTXi<Output<MODE>> {
                    fn is_set_high(&self) -> bool {
                        !self.is_set_low()
                    }

                    fn is_set_low(&self) -> bool {
                        unsafe { (*$PTX::ptr()).pdor.read().bits() & (1 << $i) == 0 }
                    }
                }

                impl<MODE> OutputPin for $PTXi<Output<MODE>> {
                    fn set_high(&mut self) {
                        unsafe { (*$PTX::ptr()).psor.write(|w| w.bits(1 << $i)) }
                    }

                    fn set_low(&mut self) {
                        unsafe { (*$PTX::ptr()).pcor.write(|w| w.bits(1 << $i)) }
                    }
                }

                impl<MODE> toggleable::Default for $PTXi<Output<MODE>> {}

                impl<MODE> InputPin for $PTXi<Input<MODE>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        unsafe { (*$PTX::ptr()).pdir.read().bits() & (1 << $i) == 0 }
                    }
                }
            )+
        }
    }
}

// Reference: 10.3.1 K20 Signal Multiplexing and Pin Assignments
gpio!(PORTA, porta, PTA, pta, gpioa, "A", [
      PTA0: (pta0, 0, Input<Floating>, "0"),
      PTA1: (pta1, 1, Input<Floating>, "1"),
      PTA2: (pta2, 2, Input<Floating>, "2"),
      PTA3: (pta3, 3, Input<Floating>, "3"),
      PTA4: (pta4, 4, Input<Floating>, "4"),
      PTA5: (pta5, 5, Input<Floating>, "5"),
      PTA12: (pta12, 12, Input<Floating>, "12"),
      PTA13: (pta13, 13, Input<Floating>, "13"),
      PTA18: (pta18, 18, Input<Floating>, "18"),
      PTA19: (pta19, 19, Input<Floating>, "19"),
]);

gpio!(PORTB, portb, PTB, ptb, gpiob, "B", [
      PTB0: (ptb0, 0, Input<Floating>, "0"),
      PTB1: (ptb1, 1, Input<Floating>, "1"),
      PTB2: (ptb2, 2, Input<Floating>, "2"),
      PTB3: (ptb3, 3, Input<Floating>, "3"),
      PTB16: (ptb16, 16, Input<Floating>, "16"),
      PTB17: (ptb17, 17, Input<Floating>, "17"),
      PTB18: (ptb18, 18, Input<Floating>, "18"),
      PTB19: (ptb19, 19, Input<Floating>, "19"),
]);

gpio!(PORTC, portc, PTC, ptc, gpioc, "C", [
      PTC0: (ptc0, 0, Input<Floating>, "0"),
      PTC1: (ptc1, 1, Input<Floating>, "1"),
      PTC2: (ptc2, 2, Input<Floating>, "2"),
      PTC3: (ptc3, 3, Input<Floating>, "3"),
      PTC4: (ptc4, 4, Input<Floating>, "4"),
      PTC5: (ptc5, 5, Input<Floating>, "5"),
      PTC6: (ptc6, 6, Input<Floating>, "6"),
      PTC7: (ptc7, 7, Input<Floating>, "7"),
      PTC8: (ptc8, 8, Input<Floating>, "8"),
      PTC9: (ptc9, 9, Input<Floating>, "9"),
      PTC10: (ptc10, 10, Input<Floating>, "10"),
      PTC11: (ptc11, 11, Input<Floating>, "11"),
]);

gpio!(PORTD, portd, PTD, ptd, gpiod, "D", [
      PTD0: (ptd0, 0, Input<Floating>, "0"),
      PTD1: (ptd1, 1, Input<Floating>, "1"),
      PTD2: (ptd2, 2, Input<Floating>, "2"),
      PTD3: (ptd3, 3, Input<Floating>, "3"),
      PTD4: (ptd4, 4, Input<Floating>, "4"),
      PTD5: (ptd5, 5, Input<Floating>, "5"),
      PTD6: (ptd6, 6, Input<Floating>, "6"),
      PTD7: (ptd7, 7, Input<Floating>, "7"),
]);

gpio!(PORTE, porte, PTE, pte, gpioe, "E", [
      PTE0: (pte0, 0, Input<Floating>, "0"),
      PTE1: (pte1, 1, Input<Floating>, "1"),
]);
