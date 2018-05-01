use mk20d7::{self, wdog::RegisterBlock};
use cortex_m::asm;

pub struct Watchdog<'a> {
    wdog: &'a RegisterBlock,
}

impl<'a> Watchdog<'a> {
    pub fn new (wdog: &'a RegisterBlock) -> Watchdog<'a> {
        Watchdog {
            wdog: wdog,
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self.wdog.stctrlh.read().wdogen() {
            // WDOG is disabled
            mk20d7::wdog::stctrlh::WDOGENR::_0 => false,

            // WDOG is enabled
            mk20d7::wdog::stctrlh::WDOGENR::_1 => true,
        }
    }

    pub fn allow_update(&self) -> bool {
        match self.wdog.stctrlh.read().allowupdate() {
            // No further updates allowed to WDOG write once registers.
            mk20d7::wdog::stctrlh::ALLOWUPDATER::_0 => false,

            // WDOG write once registers can be unlocked for updating.
            mk20d7::wdog::stctrlh::ALLOWUPDATER::_1 => true,
        }
    }

    pub fn enable(&self) {
        self.wdog.stctrlh.write(
            |w| {
                w.wdogen().set_bit()
            }
        );
    }

    pub fn disable(&self) {
        self.wdog.unlock.write(
            |w| {
                unsafe {
                    w.bits(0xC520).bits(0xD928)
                }
            }
        );

        asm::nop();
        asm::nop();

        self.wdog.stctrlh.write(
            |w| {
                w.wdogen().clear_bit()
            }
        );
    }
}
