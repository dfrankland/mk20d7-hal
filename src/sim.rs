use mk20d7::{sim::RegisterBlock, sim::clkdiv1};

const MAXIMUM_CLOCK_FREQUENCY: i8 = 72;

pub struct SystemIntegrationModule<'a> {
    sim: &'a RegisterBlock,
}

impl<'a> SystemIntegrationModule<'a> {
    pub fn new (sim: &'a RegisterBlock) -> SystemIntegrationModule<'a> {
        SystemIntegrationModule { sim }
    }

    pub fn set_dividers(&mut self, core: i8, bus: i8, flash: i8) {
        self.sim.clkdiv1.write(
            |w| {
                {
                    let core_w = w.outdiv1();
                    match core {
                        1 => core_w._0000(),
                        2 => core_w._0001(),
                        3 => core_w._0010(),
                        4 => core_w._0011(),
                        5 => core_w._0100(),
                        6 => core_w._0101(),
                        7 => core_w._0110(),
                        8 => core_w._0111(),
                        9 => core_w._1000(),
                        10 => core_w._1001(),
                        11 => core_w._1010(),
                        12 => core_w._1011(),
                        13 => core_w._1100(),
                        14 => core_w._1101(),
                        15 => core_w._1110(),
                        16 => core_w._1111(),
                        _ => panic!("Must use core value in range [1, 16]!"),
                    };
                }

                {
                    let bus_w = w.outdiv2();
                    match bus {
                        1 => bus_w._0000(),
                        2 => bus_w._0001(),
                        3 => bus_w._0010(),
                        4 => bus_w._0011(),
                        5 => bus_w._0100(),
                        6 => bus_w._0101(),
                        7 => bus_w._0110(),
                        8 => bus_w._0111(),
                        9 => bus_w._1000(),
                        10 => bus_w._1001(),
                        11 => bus_w._1010(),
                        12 => bus_w._1011(),
                        13 => bus_w._1100(),
                        14 => bus_w._1101(),
                        15 => bus_w._1110(),
                        16 => bus_w._1111(),
                        _ => panic!("Must use bus value in range [1, 16]!"),
                    };
                }

                {
                    let flash_w = w.outdiv4();
                    match flash {
                        1 => flash_w._0000(),
                        2 => flash_w._0001(),
                        3 => flash_w._0010(),
                        4 => flash_w._0011(),
                        5 => flash_w._0100(),
                        6 => flash_w._0101(),
                        7 => flash_w._0110(),
                        8 => flash_w._0111(),
                        9 => flash_w._1000(),
                        10 => flash_w._1001(),
                        11 => flash_w._1010(),
                        12 => flash_w._1011(),
                        13 => flash_w._1100(),
                        14 => flash_w._1101(),
                        15 => flash_w._1110(),
                        16 => flash_w._1111(),
                        _ => panic!("Must use flash value in range [1, 16]!"),
                    };
                }

                w
            }
        )
    }

    pub fn get_dividers(&self) -> (i8, i8, i8) {
        let r = self.sim.clkdiv1.read();

        let core = match r.outdiv1() {
            clkdiv1::OUTDIV1R::_0000 => 1,
            clkdiv1::OUTDIV1R::_0001 => 2,
            clkdiv1::OUTDIV1R::_0010 => 3,
            clkdiv1::OUTDIV1R::_0011 => 4,
            clkdiv1::OUTDIV1R::_0100 => 5,
            clkdiv1::OUTDIV1R::_0101 => 6,
            clkdiv1::OUTDIV1R::_0110 => 7,
            clkdiv1::OUTDIV1R::_0111 => 8,
            clkdiv1::OUTDIV1R::_1000 => 9,
            clkdiv1::OUTDIV1R::_1001 => 10,
            clkdiv1::OUTDIV1R::_1010 => 11,
            clkdiv1::OUTDIV1R::_1011 => 12,
            clkdiv1::OUTDIV1R::_1100 => 13,
            clkdiv1::OUTDIV1R::_1101 => 14,
            clkdiv1::OUTDIV1R::_1110 => 15,
            clkdiv1::OUTDIV1R::_1111 => 16,
        };

        let bus = match r.outdiv2() {
            clkdiv1::OUTDIV2R::_0000 => 1,
            clkdiv1::OUTDIV2R::_0001 => 2,
            clkdiv1::OUTDIV2R::_0010 => 3,
            clkdiv1::OUTDIV2R::_0011 => 4,
            clkdiv1::OUTDIV2R::_0100 => 5,
            clkdiv1::OUTDIV2R::_0101 => 6,
            clkdiv1::OUTDIV2R::_0110 => 7,
            clkdiv1::OUTDIV2R::_0111 => 8,
            clkdiv1::OUTDIV2R::_1000 => 9,
            clkdiv1::OUTDIV2R::_1001 => 10,
            clkdiv1::OUTDIV2R::_1010 => 11,
            clkdiv1::OUTDIV2R::_1011 => 12,
            clkdiv1::OUTDIV2R::_1100 => 13,
            clkdiv1::OUTDIV2R::_1101 => 14,
            clkdiv1::OUTDIV2R::_1110 => 15,
            clkdiv1::OUTDIV2R::_1111 => 16,
        };

        let flash = match r.outdiv4() {
            clkdiv1::OUTDIV4R::_0000 => 1,
            clkdiv1::OUTDIV4R::_0001 => 2,
            clkdiv1::OUTDIV4R::_0010 => 3,
            clkdiv1::OUTDIV4R::_0011 => 4,
            clkdiv1::OUTDIV4R::_0100 => 5,
            clkdiv1::OUTDIV4R::_0101 => 6,
            clkdiv1::OUTDIV4R::_0110 => 7,
            clkdiv1::OUTDIV4R::_0111 => 8,
            clkdiv1::OUTDIV4R::_1000 => 9,
            clkdiv1::OUTDIV4R::_1001 => 10,
            clkdiv1::OUTDIV4R::_1010 => 11,
            clkdiv1::OUTDIV4R::_1011 => 12,
            clkdiv1::OUTDIV4R::_1100 => 13,
            clkdiv1::OUTDIV4R::_1101 => 14,
            clkdiv1::OUTDIV4R::_1110 => 15,
            clkdiv1::OUTDIV4R::_1111 => 16,
        };

        (core, bus, flash)
    }
}
