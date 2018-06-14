use core::convert::TryFrom;

use mk20d7::{mcg::RegisterBlock, mcg::c1};

use sim::MAXIMUM_CLOCK_FREQUENCY;
use bitrate::{U32BitrateExt, KiloHertz, MegaHertz};

pub const FLL_RANGE_MIN: f32 = 31.25;
pub const FLL_RANGE_MAX: f32 = 39.0625;

pub const PLL_DIVIDER_NUMERATOR_MIN: u8 = 24;
pub const PLL_DIVIDER_NUMERATOR_MAX: u8 = 55;
pub const PLL_DIVIDER_DENOMINATOR_MIN: u8 = 1;
pub const PLL_DIVIDER_DENOMINATOR_MAX: u8 = 25;

pub struct MultipurposeClockGenerator<'a> {
    mcg: &'a RegisterBlock,
    pub external_crystal_frequency: MegaHertz<u32>,
}

pub struct Fei<'a> { mcg: &'a mut MultipurposeClockGenerator<'a> }
#[allow(dead_code)] pub struct Fee<'a> { mcg: &'a mut MultipurposeClockGenerator<'a> }
#[allow(dead_code)] pub struct Fbi<'a> { mcg: &'a mut MultipurposeClockGenerator<'a> }
pub struct Fbe<'a> { mcg: &'a mut MultipurposeClockGenerator<'a> }
pub struct Pee<'a> { #[allow(dead_code)] mcg: &'a mut MultipurposeClockGenerator<'a> }
pub struct Pbe<'a> { mcg: &'a mut MultipurposeClockGenerator<'a> }
#[allow(dead_code)] pub struct Blpi<'a> { mcg: &'a mut MultipurposeClockGenerator<'a> }
#[allow(dead_code)] pub struct Blpe<'a> { mcg: &'a mut MultipurposeClockGenerator<'a> }
#[allow(dead_code)] pub struct Stop<'a> { mcg: &'a mut MultipurposeClockGenerator<'a> }

// Multipurpose Clock Generator (MCG) modes of operation
pub enum ClockMode<'a> {
    Fei(Fei<'a>), // FEI: Frequency Locked Loop (FLL) Engaged Internal
    Fee(Fee<'a>), // FEE: Frequency Locked Loop (FLL) Engaged External
    Fbi(Fbi<'a>), // FBI: Frequency Locked Loop (FLL) Bypassed Internal
    Fbe(Fbe<'a>), // FBE: Frequency Locked Loop (FLL) Bypassed External
    Pee(Pee<'a>), // PEE: Phase Locked Loop (PLL) Engaged External
    Pbe(Pbe<'a>), // PBE: Phase Locked Loop (PLL) Bypassed External
    Blpi(Blpi<'a>), // BLPI: Bypassed Low Power Internal
    Blpe(Blpe<'a>), // BLPE: Bypassed Low Power External
    Stop(Stop<'a>), // Stop
}

impl<'a> MultipurposeClockGenerator<'a> {
    pub fn new(mcg: &'a RegisterBlock, external_crystal_frequency: MegaHertz<u32>) -> MultipurposeClockGenerator<'a> {
        MultipurposeClockGenerator { mcg, external_crystal_frequency }
    }

    pub fn clock_mode(&'a mut self) -> ClockMode<'a> {
        let clock_source = self.mcg.c1.read().clks();
        let internal_clock_reference = self.mcg.c1.read().irefs().bit_is_set();
        let pll_enabled = self.mcg.c6.read().plls().bit_is_set();
        let low_power_enabled = self.mcg.c2.read().lp().bit_is_set();

        let external_crystal_frequency_khz: KiloHertz<u32> = self.external_crystal_frequency.into();
        let fll = external_crystal_frequency_khz.0 as f32 / f32::from(self.get_external_crystal_frequency_divider());
        let fll_range_ok = fll >= FLL_RANGE_MIN && fll <= FLL_RANGE_MAX;

        let mcg = self;
        match (clock_source, internal_clock_reference, pll_enabled, low_power_enabled, fll_range_ok) {
            (c1::CLKSR::_00, true, false, _, _) => ClockMode::Fei(Fei { mcg }),
            (c1::CLKSR::_00, false, false, _, true) => ClockMode::Fee(Fee { mcg }),
            (c1::CLKSR::_01, true, false, false, _) => ClockMode::Fbi(Fbi { mcg }),
            (c1::CLKSR::_10, false, false, false, true) => ClockMode::Fbe(Fbe { mcg }),
            (c1::CLKSR::_00, false, true, _, _) => ClockMode::Pee(Pee { mcg }),
            (c1::CLKSR::_10, false, true, false, _) => ClockMode::Pbe(Pbe { mcg }),
            (c1::CLKSR::_01, true, false, true, _) => ClockMode::Blpi(Blpi { mcg }),
            (c1::CLKSR::_10, false, _, true, _) => ClockMode::Blpe(Blpe { mcg }),
            _ => panic!("The current clock mode cannot be represented as a known struct"),
        }
    }

    pub fn external_crystal_is_requested(&self) -> bool {
        self.mcg.c2.read().erefs0().bit_is_set()
    }

    pub fn enable_external_crystal_request(&mut self) {
        if self.external_crystal_is_requested() { return; }
        self.mcg.c2.write(|w| w.erefs0().set_bit());
        while self.mcg.s.read().oscinit0().bit_is_clear() {} // Wait to become enabled
    }

    pub fn disable_external_crystal_request(&mut self) {
        if !self.external_crystal_is_requested() { return; }
        self.mcg.c2.write(|w| w.erefs0().clear_bit());
        while self.mcg.s.read().oscinit0().bit_is_set() {} // Wait to become disabled
    }

    pub fn set_external_crystal_frequency_range_low(&mut self) {
        self.mcg.c2.write(|w| w.range0()._00());
    }

    pub fn set_external_crystal_frequency_range_high(&mut self) {
        self.mcg.c2.write(|w| w.range0()._01());
    }

    pub fn set_external_crystal_frequency_divider(&self, divider: u16) {
        let crystal_low_frequency = self.mcg.c2.read().range0().is_00();
        let real_time_clock = self.mcg.c7.read().oscsel().bit_is_set();
        let rtc_or_low_freq_crystal = crystal_low_frequency || real_time_clock;

        self.mcg.c1.write(
            |w| {
                let frdiv_w = w.frdiv();
                match divider {
                    _ if rtc_or_low_freq_crystal && divider == 1 || divider == 32 => frdiv_w._000(),
                    _ if rtc_or_low_freq_crystal && divider == 2 || divider == 64 => frdiv_w._001(),
                    _ if rtc_or_low_freq_crystal && divider == 4 || divider == 128 => frdiv_w._010(),
                    _ if rtc_or_low_freq_crystal && divider == 8 || divider == 256 => frdiv_w._011(),
                    _ if rtc_or_low_freq_crystal && divider == 16 || divider == 512 => frdiv_w._100(),
                    _ if rtc_or_low_freq_crystal && divider == 32 || divider == 1024 => frdiv_w._101(),
                    _ if rtc_or_low_freq_crystal && divider == 64 || divider == 1280 => frdiv_w._110(),
                    _ if rtc_or_low_freq_crystal && divider == 128 || divider == 1536 => frdiv_w._111(),
                    _ => panic!("Invalid external clock divider: {}", divider),
                }
            }
        );
    }

    pub fn get_external_crystal_frequency_divider(&self) -> u16 {
        let crystal_low_frequency = self.mcg.c2.read().range0().is_00();
        let real_time_clock = self.mcg.c7.read().oscsel().bit_is_set();
        let rtc_or_low_freq_crystal = crystal_low_frequency || real_time_clock;

        match self.mcg.c1.read().frdiv() {
            c1::FRDIVR::_000 => if rtc_or_low_freq_crystal { 1 } else { 32 },
            c1::FRDIVR::_001 => if rtc_or_low_freq_crystal { 2 } else { 64 },
            c1::FRDIVR::_010 => if rtc_or_low_freq_crystal { 4 } else { 128 },
            c1::FRDIVR::_011 => if rtc_or_low_freq_crystal { 8 } else { 256 },
            c1::FRDIVR::_100 => if rtc_or_low_freq_crystal { 16 } else { 512 },
            c1::FRDIVR::_101 => if rtc_or_low_freq_crystal { 32 } else { 1024 },
            c1::FRDIVR::_110 => if rtc_or_low_freq_crystal { 64 } else { 1280 },
            c1::FRDIVR::_111 => if rtc_or_low_freq_crystal { 128 } else { 1536 },
        }
    }

    pub fn use_external_crystal(&mut self) {
        self.mcg.c1.write(
            |w| {
                w.clks()._10();
                w.irefs().clear_bit()
            }
        );

        // Once we write to the control register, we need to wait for
        // the new clock to stabilize before we move on.
        while self.mcg.s.read().irefst().bit_is_set() {} // Wait for FLL to point to the crystal
        while !self.mcg.s.read().clkst().is_10() {} // Wait for clock source to be the crystal osc
    }

    pub fn set_pll_frequency_divider(&mut self, numerator: u8, denominator: u8) {
        if numerator < PLL_DIVIDER_NUMERATOR_MIN || numerator > PLL_DIVIDER_NUMERATOR_MAX {
            panic!("Invalid PLL VCO divide factor: {}", numerator);
        }

        if denominator < PLL_DIVIDER_DENOMINATOR_MIN || denominator > PLL_DIVIDER_DENOMINATOR_MAX {
            panic!("Invalid PLL reference divide factor: {}", denominator);
        }

        self.mcg.c5.write(|w| unsafe { w.prdiv0().bits(denominator - PLL_DIVIDER_DENOMINATOR_MIN) });
        self.mcg.c6.write(|w| unsafe { w.vdiv0().bits(numerator - PLL_DIVIDER_NUMERATOR_MIN) });
    }

    pub fn get_pll_frequency_divider(&self) -> (u8, u8) {
        let numerator = self.mcg.c6.read().vdiv0().bits() + PLL_DIVIDER_NUMERATOR_MIN;
        let denominator = self.mcg.c5.read().prdiv0().bits() + PLL_DIVIDER_DENOMINATOR_MIN;
        (numerator, denominator)
    }

    pub fn set_pll_frequency(&mut self, frequency: MegaHertz<u32>) {
        let divider = pll_frequency_divider_gcd(
            u8::try_from(frequency.0).unwrap(),
            u8::try_from(self.external_crystal_frequency.0).unwrap()
        );
        self.set_pll_frequency_divider(divider.0, divider.1);
    }

    pub fn get_pll_frequency(&self) -> MegaHertz<u32> {
        let (numerator, denominator) = self.get_pll_frequency_divider();
        let num = u32::from(numerator);
        let den = u32::from(denominator);
        ((num * self.external_crystal_frequency.0) / den).mhz()
    }

    pub fn enable_pll(&mut self) {
        self.mcg.c6.write(|w| w.plls().set_bit());
        while self.mcg.s.read().pllst().bit_is_clear() {} // Wait for PLL to be enabled
        while self.mcg.s.read().lock0().bit_is_clear() {} // Wait for PLL to be "locked" and stable
    }

    pub fn use_pll(&mut self) {
        self.mcg.c1.write(|w| w.clks()._10());

        // mcg.c1 and mcg.s have slightly different behaviors. In c1, we use one value to indicate
        // "Use whichever LL is enabled". In s, it is differentiated between the FLL at 0, and the
        // PLL at 3. Instead of adding a value to OscSource which would be invalid to set, we just
        // check for the known value "3" here.
        while !self.mcg.s.read().clkst().is_10() {}
    }
}

impl<'a> Into<Fbe<'a>> for Fei<'a> {
    fn into(self) -> Fbe<'a> {
        self.mcg.set_external_crystal_frequency_range_high();
        self.mcg.enable_external_crystal_request();
        self.mcg.set_external_crystal_frequency_divider(512); // FIXME: Assumes a 16 Mhz crystal, don't hard code this
        self.mcg.use_external_crystal();
        match self.mcg.clock_mode() {
            ClockMode::Fbe(fbe) => fbe,
            _ => panic!("Somehow the clock wasn't in FBE mode"),
        }
    }
}

impl<'a> Into<Pbe<'a>> for Fbe<'a> {
    fn into(self) -> Pbe<'a> {
        self.mcg.set_pll_frequency(u32::from(MAXIMUM_CLOCK_FREQUENCY).mhz()); // FIXME: Assumes 72 Mhz, don't hard code this
        self.mcg.enable_pll();
        match self.mcg.clock_mode() {
            ClockMode::Pbe(pbe) => pbe,
            _ => panic!("Somehow the clock wasn't in PBE mode"),
        }
    }
}

impl<'a> Into<Pee<'a>> for Pbe<'a> {
    fn into(self) -> Pee<'a> {
        self.mcg.use_pll();
        match self.mcg.clock_mode() {
            ClockMode::Pee(pee) => pee,
            _ => panic!("Somehow the clock wasn't in PEE mode"),
        }
    }
}

fn pll_frequency_divider_gcd(numerator: u8, denominator: u8) -> (u8, u8) {
    // Euclid's GCD
    let mut num = numerator;
    let mut den = denominator;
    while den != 0 {
        let temp = den;
        den = num % den;
        num = temp;
    }
    let gcd = num;
    num = numerator / gcd;
    den = denominator / gcd;

    // GCD too high or too low, not a valid PLL frequency
    if num == 0 || den == 0 || num > PLL_DIVIDER_NUMERATOR_MAX || den > PLL_DIVIDER_DENOMINATOR_MAX {
        panic!("Cannot find a GCD for PLL frequency divider {}/{}.", numerator, denominator);
    }

    // GCD too low, coerce into an acceptable range
    let mut freq_num = num;
    let mut freq_den = den;
    let mut mul = 1;
    while freq_num < PLL_DIVIDER_NUMERATOR_MIN || freq_den < PLL_DIVIDER_DENOMINATOR_MIN {
        mul += 1;
        match (num.checked_mul(mul), den.checked_mul(mul)) {
            (Some(new_freq_num), Some(new_freq_den)) if
            new_freq_num <= PLL_DIVIDER_NUMERATOR_MAX &&
            new_freq_den <= PLL_DIVIDER_DENOMINATOR_MAX => {
                freq_num = new_freq_num;
                freq_den = new_freq_den;
            },
            _ => panic!("Cannot find a GCD for PLL frequency divider {}/{}.", numerator, denominator),
        }
    }

    (freq_num, freq_den)
}
