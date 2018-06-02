use cast::u32;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use hal::blocking::delay::{DelayMs, DelayUs};
use sim::SystemIntegrationModule;

pub struct Delay<'a> {
    sim: &'a SystemIntegrationModule<'a>,
    syst: SYST,
}

impl<'a> Delay<'a> {
    pub fn new(mut syst: SYST, sim: &'a SystemIntegrationModule<'a>) -> Self {
        syst.set_clock_source(SystClkSource::Core);

        Delay { syst, sim }
    }

    pub fn free(self) -> SYST {
        self.syst
    }
}

impl<'a> DelayMs<u32> for Delay<'a> {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl<'a> DelayMs<u16> for Delay<'a> {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(u32(ms));
    }
}

impl<'a> DelayMs<u8> for Delay<'a> {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u32(ms));
    }
}

impl<'a> DelayUs<u32> for Delay<'a> {
    fn delay_us(&mut self, us: u32) {
        let rvr = us * self.sim.get_frequencies().0 as u32;

        if rvr > (1 << 24) {
            panic!("Delay must be between 1 and 0x00ffffff (1 << 24).");
        }

        self.syst.set_reload(rvr);
        self.syst.clear_current();
        self.syst.enable_counter();

        while !self.syst.has_wrapped() {}

        self.syst.disable_counter();
    }
}

impl<'a> DelayUs<u16> for Delay<'a> {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(u32(us))
    }
}

impl<'a> DelayUs<u8> for Delay<'a> {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(u32(us))
    }
}
