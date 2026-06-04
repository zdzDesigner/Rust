use crate::rcc::Rcc;
use crate::volatile::VolatileCell;

#[repr(C)]
struct Regs {
    _crl: VolatileCell<u32>,
    crh: VolatileCell<u32>,
    _idr: VolatileCell<u32>,
    _odr: VolatileCell<u32>,
    bsrr: VolatileCell<u32>,
    _brr: VolatileCell<u32>,
    _lckr: VolatileCell<u32>,
}

const GPIOC_BASE: *const Regs = 0x4001_1000 as *const Regs;
const PC13_SHIFT: u32 = 20;
const PC13_OUTPUT_10MHZ_PP: u32 = 0b0001;

pub struct GpioC {
    regs: *const Regs,
}

pub struct Pc13;

pub struct Pc13Output {
    regs: *const Regs,
}

impl GpioC {
    pub const fn new() -> Self {
        Self { regs: GPIOC_BASE }
    }

    pub const fn pc13(&self) -> Pc13 {
        Pc13
    }
}

impl Pc13 {
    pub fn into_push_pull_output(self, rcc: &Rcc, port: &GpioC) -> Pc13Output {
        rcc.enable_gpioc();

        let regs = unsafe { &*port.regs };
        let current = regs.crh.get();
        let next = (current & !(0xF << PC13_SHIFT)) | (PC13_OUTPUT_10MHZ_PP << PC13_SHIFT);
        regs.crh.set(next);

        Pc13Output { regs: port.regs }
    }
}

impl Pc13Output {
    pub fn set_high(&self) {
        let regs = unsafe { &*self.regs };
        regs.bsrr.set(1 << 13);
    }

    pub fn set_low(&self) {
        let regs = unsafe { &*self.regs };
        regs.bsrr.set(1 << (13 + 16));
    }
}
