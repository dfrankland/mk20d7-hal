use core::marker::PhantomData;
use mk20d7::sim::SCGC5;

pub trait GpioExt {
    type Parts;
    fn split(self, scgc5: &SCGC5) -> Self::Parts;
}

pub struct Inactive;
pub struct Input<MODE> {
    _mode: PhantomData<MODE>
}
pub struct Floating;
pub struct Output<MODE> {
    _mode: PhantomData<MODE>
}
pub struct OpenDrain;
pub struct PushPull;

macro_rules! gpio {
    ($PORTX:ident, $portx:ident, $PTX:ident, $gpiox:ident, [ $($PTXi:ident: ($ptxi:ident, $i:expr, $MODE:ty),)+]) =>
    {
        pub mod $gpiox {
            use core::marker::PhantomData;

            use hal::digital::OutputPin;

            use mk20d7::{$PORTX, $PTX};
            // use mk20d7::$portx::PCR;
            use mk20d7::sim::SCGC5;

            use super::{Output, Inactive, PushPull, GpioExt};
            // use super::{Input, Floating, OpenDrain};

            pub struct Parts {
                //_gpclr: GPCLR,
                //_gpchr: GPCHR,
                //_isfr: ISFR,
                //_pddr: PDDR,
                $(
                pub $ptxi: $PTXi<$MODE>,
                )+
            }

            impl GpioExt for ($PTX, $PORTX) {
                type Parts = Parts;
                fn split(self, scgc5: &SCGC5) -> Self::Parts {
                    // Enable the GPIO module
                    // Reference: 10.2.3 Clock gating
                    scgc5.write(
                        |w| {
                            w.$portx().set_bit()
                        }
                    );

                    Parts {
                        //// These registers are dangerous to keep around because they can modify
                        //// pins we have already moved out
                        // _gpclr: GPCLR{},
                        // _gpchr: GPCHR{},

                        //// This is also a global register, see above
                        // _isfr: ISFR{},
                        // _pddr: PDDR{},

                        $(
                        $ptxi: $PTXi {_mode: PhantomData},
                        )+
                    }
                }
            }

            //// These come from PORTx

            ///// Global Pin Control Low Register
            // struct GPCLR {}

            ///// Global Pin Control High Register
            // struct GPCHR {}

            ///// Interrupt Status Flag Register
            // struct ISFR {}

            // I was not able to find documentation on these registers outside of the svd

            ///// Digital Filter Enable Register
            // struct DFER {}

            ///// Digital Filter Clock Register
            // struct DFCR {}

            ///// Digital Filter Width Register
            // struct DFWR {}

            // These are found in PTx

            ///// Port Data Output Register
            // struct PDOR {}

            ///// Port Set Output Register
            // struct PSOR {}

            ///// Port Clear Output Register
            // struct PCOR {}

            ///// Port Toggle Output Register
            // struct PTOR {}

            ///// Port Data Input Register
            // struct PDIR {}

            ///// Port Data Direction Register
            // struct PDDR {}

            // This pin owns its section of the PDOR, PSOR, PCOR, PTOR, and PDIR registers, as well
            // as its PCR register
            $(
                pub struct $PTXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PTXi<MODE> {
                    // pub(crate) fn pcr(&mut self) -> &PCR {
                    //     unsafe {
                    //         &(*$PORTX::ptr()).pcr[$i]
                    //     }
                    // }

                    pub fn into_push_pull_output(self) -> $PTXi<Output<PushPull>> {
                        // Set the pin to mode 1 (GPIO), and disable Open Drain mode
                        unsafe {
                            (*$PORTX::ptr()).pcr[$i].write(
                                |w| {
                                    w.mux()._001().ode().clear_bit().dse().set_bit().sre().set_bit()
                                }
                            )
                        };

                        // Set the pin to output mode
                        unsafe {
                            (*$PTX::ptr()).pddr.modify(
                                |r, w| {
                                    w.bits(r.bits() | 1 << $i)
                                }
                            )
                        };

                        $PTXi { _mode: PhantomData }
                    }
                }

                impl<MODE> OutputPin for $PTXi<Output<MODE>> {
                    fn is_high(&self) -> bool {
                        let output = unsafe {
                            (*$PTX::ptr()).pdor.read().bits()
                        };
                        output & (1 << $i) != 0
                    }

                    fn is_low(&self) -> bool {
                        !self.is_high()
                    }

                    fn set_low(&mut self) {
                        unsafe {
                            (*$PTX::ptr()).pcor.write(
                                |w| {
                                    w.bits(1 << $i)
                                }
                            )
                        };
                    }

                    fn set_high(&mut self) {
                        unsafe {
                            (*$PTX::ptr()).psor.write(
                                |w| {
                                    w.bits(1 << $i)
                                }
                            )
                        };
                    }
                }
            )+
        }
    }
}

// Reference: 10.3.1 K20 Signal Multiplexing and Pin Assignments
gpio!(PORTA, porta, PTA, gpioa, [
      PTA0: (pta0, 0, Inactive), // JTAG_TCLK / SWD_CLK / EZP_CLK
      PTA1: (pta1, 1, Inactive), // JTAG_TDI / EZP_DI
      PTA2: (pta2, 2, Inactive), // JTAG_TDO / TRACE_SWO / EZP_DO
      PTA3: (pta3, 3, Inactive), // JTAG_TMS / SWD_DIO
      PTA4: (pta4, 4, Inactive), // NMI_b / EZP_CS_b
      PTA5: (pta5, 5, Inactive), // Disabled
      PTA12: (pta12, 12, Inactive), // CMP2_IN0
      PTA13: (pta13, 13, Inactive), // CMP2_IN1
      PTA18: (pta18, 18, Inactive), // EXTAL0
      PTA19: (pta19, 19, Inactive), // XTAL0
]);

gpio!(PORTB, portb, PTB, gpiob, [
      PTB0: (ptb0, 0, Inactive), // ADC0_SE8 / ADC1_SE8 / TSI0_CH0
      PTB1: (ptb1, 1, Inactive), // ADC0_SE9 / ADC1_SE9 / TSI0_CH6
      PTB2: (ptb2, 2, Inactive), // ADC0_SE12 / TSI0_CH7
      PTB3: (ptb3, 3, Inactive), // ADC0_SE13 / TSI0_CH8
      PTB16: (ptb16, 16, Inactive), // TSI0_CH9
      PTB17: (ptb17, 17, Inactive), // TSI0_CH10
      PTB18: (ptb18, 18, Inactive), // TSI0_CH11
      PTB19: (ptb19, 19, Inactive), // TSI0_CH12
]);

gpio!(PORTC, portc, PTC, gpioc, [
      PTC0: (ptc0, 0, Inactive), // ADC0_SE14 / TSI0_CH13
      PTC1: (ptc1, 1, Inactive), // ADC0_SE15 / TSI0_CH14
      PTC2: (ptc2, 2, Inactive), // ADC0_SE4b / CMP1_IN0 / TSI0_CH15
      PTC3: (ptc3, 3, Inactive), // CMP1_IN1
      PTC4: (ptc4, 4, Inactive), // Disabled
      PTC5: (ptc5, 5, Inactive), // Disabled
      PTC6: (ptc6, 6, Inactive), // CMP0_IN0
      PTC7: (ptc7, 7, Inactive), // CMP0_IN1
      PTC8: (ptc8, 8, Inactive), // ADC1_SE4b / CMP0_IN2
      PTC9: (ptc9, 9, Inactive), // ADC1_SE5b / CMP0_IN3
      PTC10: (ptc10, 10, Inactive), // ADC1_SE6b
      PTC11: (ptc11, 11, Inactive), // ADC1_SE7b
]);

gpio!(PORTD, portd, PTD, gpiod, [
      PTD0: (ptd0, 0, Inactive), // Disabled
      PTD1: (ptd1, 1, Inactive), // ADC0_SE5b
      PTD2: (ptd2, 2, Inactive), // Disabled
      PTD3: (ptd3, 3, Inactive), // Disabled
      PTD4: (ptd4, 4, Inactive), // Disabled
      PTD5: (ptd5, 5, Inactive), // ADC0_SE6b
      PTD6: (ptd6, 6, Inactive), // ADC0_SE7b
      PTD7: (ptd7, 7, Inactive), // Disabled
]);

gpio!(PORTE, porte, PTE, gpioe, [
      PTE0: (pte0, 0, Inactive), // ADC1_SE4a
      PTE1: (pte1, 1, Inactive), // ADC1_SE5a
]);
