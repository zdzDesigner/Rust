use crate::rcc::Rcc;
use crate::volatile::VolatileCell;
use core::marker::PhantomData;

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
const GPIOB_BASE: *const Regs = 0x4001_0C00 as *const Regs;
const OUTPUT_10MHZ_PP: u32 = 0b0001;

pub trait GpioPort {
    fn enable(rcc: &Rcc);
    fn regs(&self) -> *const ();
}

fn crh_shift(pin: u8) -> u32 {
    debug_assert!((8..=15).contains(&pin));
    u32::from(pin - 8) * 4
}

fn configure_push_pull_output(regs: *const Regs, pin: u8) {
    let regs = unsafe { &*regs };
    let shift = crh_shift(pin);
    regs.crh
        .update(|value| (value & !(0xF << shift)) | (OUTPUT_10MHZ_PP << shift));
}

pub struct Pin<const N: u8, P> {
    _port: PhantomData<P>,
}

impl<const N: u8, P> Pin<N, P> {
    const fn new() -> Self {
        Self { _port: PhantomData }
    }
}

impl<const N: u8, P: GpioPort> Pin<N, P> {
    pub fn into_push_pull_output(self, rcc: &Rcc, port: &P) -> Output<N, P> {
        P::enable(rcc);
        let regs = port.regs().cast();
        configure_push_pull_output(regs, N);
        Output {
            regs,
            _port: PhantomData,
        }
    }
}

pub struct Output<const N: u8, P> {
    regs: *const Regs,
    _port: PhantomData<P>,
}

impl<const N: u8, P> Output<N, P> {
    pub fn set_high(&self) {
        let regs = unsafe { &*self.regs };
        regs.bsrr.write(1 << N);
    }

    pub fn set_low(&self) {
        let regs = unsafe { &*self.regs };
        regs.bsrr.write(1 << (u32::from(N) + 16));
    }
}

macro_rules! gpio_port {
    ($port:ident, $base:ident, $enable:ident, { $($pin_fn:ident => ($pin_ty:ident, $out_ty:ident, $n:literal)),+ $(,)? }) => {
        pub struct $port {
            regs: *const Regs,
        }

        impl GpioPort for $port {
            fn enable(rcc: &Rcc) {
                rcc.$enable();
            }

            fn regs(&self) -> *const () {
                self.regs.cast()
            }
        }

        impl $port {
            pub const fn new() -> Self {
                Self { regs: $base }
            }

            $(
                pub const fn $pin_fn(&self) -> $pin_ty {
                    Pin::new()
                }
            )+
        }

        $(
            pub type $pin_ty = Pin<$n, $port>;
            pub type $out_ty = Output<$n, $port>;
        )+
    };
}

gpio_port!(GpioC, GPIOC_BASE, enable_gpioc, {
    pc13 => (Pc13, Pc13Output, 13),
});

gpio_port!(GpioB, GPIOB_BASE, enable_gpiob, {
    pb12 => (Pb12, Pb12Output, 12),
});
