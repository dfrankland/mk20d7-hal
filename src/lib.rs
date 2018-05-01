#![feature(never_type)]
#![no_std]

extern crate cortex_m;
extern crate embedded_hal as hal;
pub extern crate mk20d7;

pub mod gpio;
pub mod osc;
pub mod prelude;
pub mod wdog;
