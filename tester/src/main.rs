#![allow(unused_macros)]

macro_rules! create_function {
    ($func_name: ident) => {
        fn $func_name() {
            println!("{:?}() was called!", stringify!($func_name));
        }
    };
}

macro_rules! print_result {
    ($expression: expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

macro_rules! macro_test {
    ($({expression: $arg: expr},)+) =>{
        println!("{:?}", ($($arg),+));
    };
}

macro_rules! calculate {
    (eval $e:expr) => {{
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
    }};
}

create_function!(foo);
create_function!(bar);

fn main() {
    foo();
    bar();

    print_result!(1 + 2 * 4);
    print_result!({
        let mut x = 0;
        while x < 3 {
            println!("x * 2 = {}", x * 2);
            x += 1;
        }

        x
    });

    macro_test!({expression: "expr"}, {expression: 1+5+5},);

}

macro_rules! gpio {
    ([
        $({
            devices: [$($device:expr,)+],
            GPIO: $GPIOX:ident,
            gpio: $gpiox:ident,
            gpio_mapped: $gpioy:ident,
            gpio_mapped_ioenr: $iopxenr:ident,
            gpio_mapped_iorst: $iopxrst:ident,
            partially_erased_pin: $PXx:ident,
            pins: [
                $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty, $AFR:ident),)+
            ],
        },)+
    ]) => {
        $(
            #[cfg(any(
                $(feature = $device,)+
            ))]
            use crate::stm32::$GPIOX;
        )+

        pub enum Gpio {
            $(
                #[cfg(any(
                    $(feature = $device,)+
                ))]
                $GPIOX,
            )+
        }

        /// Fully erased pin
        pub struct PXx<MODE> {
            i: u8,
            gpio: Gpio,
            _mode: PhantomData<MODE>,
        }

        impl<MODE> OutputPin for PXx<Output<MODE>> {
            type Error = ();

            fn set_high(&mut self) -> Result<(), Self::Error> {
                // NOTE(unsafe) atomic write to a stateless register
                unsafe {
                    match &self.gpio {
                        $(
                            #[cfg(any(
                                $(feature = $device,)+
                            ))]
                            Gpio::$GPIOX => (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << self.i)),
                        )+
                    }
                }
                Ok(())
            }

            fn set_low(&mut self) -> Result<(), Self::Error> {
                // NOTE(unsafe) atomic write to a stateless register
                unsafe {
                    match &self.gpio {
                        $(
                            #[cfg(any(
                                $(feature = $device,)+
                            ))]
                            Gpio::$GPIOX => (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + self.i))),
                        )+
                    }
                }
                Ok(())
            }
        }

        #[cfg(feature = "unproven")]
        impl<MODE> InputPin for PXx<Input<MODE>> {
            type Error = ();

            fn is_high(&self) -> Result<bool, Self::Error> {
                Ok(!self.is_low()?)
            }

             fn is_low(&self) -> Result<bool, Self::Error> {
                // NOTE(unsafe) atomic read with no side effects
                Ok(unsafe {
                    match &self.gpio {
                        $(
                            #[cfg(any(
                                $(feature = $device,)+
                            ))]
                            Gpio::$GPIOX => (*$GPIOX::ptr()).idr.read().bits() & (1 << self.i) == 0,
                        )+
                    }
                })
            }
        }

        $(
            /// GPIO
            #[cfg(any(
                $(feature = $device,)+
            ))]
            pub mod $gpiox {
                use core::marker::PhantomData;

                use crate::hal::digital::v2::OutputPin;
                #[cfg(feature = "unproven")]
                use crate::hal::digital::v2::InputPin;
                use crate::stm32::{$gpioy, $GPIOX};

                use crate::rcc::AHB;
                use super::{
                    AF4, AF5, AF6, AF7, AF14, Floating, GpioExt, Input, OpenDrain, Output,
                    PullDown, PullUp, PushPull,
                    PXx, Gpio,
                };

                /// GPIO parts
                pub struct Parts {
                    /// Opaque AFRH register
                    pub afrh: AFRH,
                    /// Opaque AFRL register
                    pub afrl: AFRL,
                    /// Opaque MODER register
                    pub moder: MODER,
                    /// Opaque OTYPER register
                    pub otyper: OTYPER,
                    /// Opaque PUPDR register
                    pub pupdr: PUPDR,
                    $(
                        /// Pin
                        pub $pxi: $PXi<$MODE>,
                    )+
                }

                impl GpioExt for $GPIOX {
                    type Parts = Parts;

                    fn split(self, ahb: &mut AHB) -> Parts {
                        ahb.enr().modify(|_, w| w.$iopxenr().set_bit());
                        ahb.rstr().modify(|_, w| w.$iopxrst().set_bit());
                        ahb.rstr().modify(|_, w| w.$iopxrst().clear_bit());

                        Parts {
                            afrh: AFRH { _0: () },
                            afrl: AFRL { _0: () },
                            moder: MODER { _0: () },
                            otyper: OTYPER { _0: () },
                            pupdr: PUPDR { _0: () },
                            $(
                                $pxi: $PXi { _mode: PhantomData },
                            )+
                        }
                    }
                }

                /// Opaque AFRL register
                pub struct AFRL {
                    _0: (),
                }

                impl AFRL {
                    pub(crate) fn afr(&mut self) -> &$gpioy::AFRL {
                        unsafe { &(*$GPIOX::ptr()).afrl }
                    }
                }

                /// Opaque AFRH register
                pub struct AFRH {
                    _0: (),
                }

                impl AFRH {
                    // stm32f301 and stm32f318 don't have any high pins for GPIOF
                    #[allow(dead_code)]
                    pub(crate) fn afr(&mut self) -> &$gpioy::AFRH {
                        unsafe { &(*$GPIOX::ptr()).afrh }
                    }
                }

                /// Opaque MODER register
                pub struct MODER {
                    _0: (),
                }

                impl MODER {
                    pub(crate) fn moder(&mut self) -> &$gpioy::MODER {
                        unsafe { &(*$GPIOX::ptr()).moder }
                    }
                }

                /// Opaque OTYPER register
                pub struct OTYPER {
                    _0: (),
                }

                impl OTYPER {
                    pub(crate) fn otyper(&mut self) -> &$gpioy::OTYPER {
                        unsafe { &(*$GPIOX::ptr()).otyper }
                    }
                }

                /// Opaque PUPDR register
                pub struct PUPDR {
                    _0: (),
                }

                impl PUPDR {
                    pub(crate) fn pupdr(&mut self) -> &$gpioy::PUPDR {
                        unsafe { &(*$GPIOX::ptr()).pupdr }
                    }
                }

                /// Partially erased pin
                pub struct $PXx<MODE> {
                    i: u8,
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXx<MODE> {
                    /// Erases the port letter from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> PXx<MODE> {
                        PXx {
                            i: self.i,
                            gpio: Gpio::$GPIOX,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> OutputPin for $PXx<Output<MODE>> {
                    type Error = ();

                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << self.i)) }
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + self.i))) }
                        Ok(())
                    }
                }

                #[cfg(feature = "unproven")]
                impl<MODE> InputPin for $PXx<Input<MODE>> {
                    type Error = ();

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        Ok(!self.is_low()?)
                    }

                     fn is_low(&self) -> Result<bool, Self::Error> {
                        // NOTE(unsafe) atomic read with no side effects
                        Ok(unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << self.i) == 0 })
                    }
                }

                $(
                    /// Pin
                    pub struct $PXi<MODE> {
                        _mode: PhantomData<MODE>,
                    }

                    impl<MODE> $PXi<MODE> {
                        /// Configures the pin to serve as alternate function 4 (AF4)
                        pub fn into_af4(
                            self,
                            moder: &mut MODER,
                            afr: &mut $AFR,
                        ) -> $PXi<AF4> {
                            let offset = 2 * $i;

                            // alternate function mode
                            let mode = 0b10;
                            moder.moder().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                            });

                            let af = 4;
                            let offset = 4 * ($i % 8);
                            afr.afr().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                            });

                            $PXi { _mode: PhantomData }
                        }

                        /// Configures the pin to serve as alternate function 5 (AF5)
                        pub fn into_af5(
                            self,
                            moder: &mut MODER,
                            afr: &mut $AFR,
                        ) -> $PXi<AF5> {
                            let offset = 2 * $i;

                            // alternate function mode
                            let mode = 0b10;
                            moder.moder().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                            });

                            let af = 5;
                            let offset = 4 * ($i % 8);
                            afr.afr().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                            });

                            $PXi { _mode: PhantomData }
                        }

                        /// Configures the pin to serve as alternate function 6 (AF6)
                        pub fn into_af6(
                            self,
                            moder: &mut MODER,
                            afr: &mut $AFR,
                        ) -> $PXi<AF6> {
                            let offset = 2 * $i;

                            // alternate function mode
                            let mode = 0b10;
                            moder.moder().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                            });

                            let af = 6;
                            let offset = 4 * ($i % 8);
                            afr.afr().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                            });

                            $PXi { _mode: PhantomData }
                        }

                        /// Configures the pin to serve as alternate function 7 (AF7)
                        pub fn into_af7(
                            self,
                            moder: &mut MODER,
                            afr: &mut $AFR,
                        ) -> $PXi<AF7> {
                            let offset = 2 * $i;

                            // alternate function mode
                            let mode = 0b10;
                            moder.moder().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                            });

                            let af = 7;
                            let offset = 4 * ($i % 8);

                            afr.afr().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                            });

                            $PXi { _mode: PhantomData }
                        }

                        /// Configures the pin to serve as alternate function 14 (AF14)
                        pub fn into_af14(
                            self,
                            moder: &mut MODER,
                            afr: &mut $AFR,
                        ) -> $PXi<AF14> {
                            let offset = 2 * $i;

                            // alternate function mode
                            let mode = 0b10;
                            moder.moder().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                            });

                            let af = 14;
                            let offset = 4 * ($i % 8);
                            afr.afr().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b1111 << offset)) | (af << offset))
                            });

                            $PXi { _mode: PhantomData }
                        }

                        /// Configures the pin to operate as a floating input pin
                        pub fn into_floating_input(
                            self,
                            moder: &mut MODER,
                            pupdr: &mut PUPDR,
                        ) -> $PXi<Input<Floating>> {
                            let offset = 2 * $i;

                            // input mode
                            moder
                                .moder()
                                .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                            // no pull-up or pull-down
                            pupdr
                                .pupdr()
                                .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                            $PXi { _mode: PhantomData }
                        }

                        /// Configures the pin to operate as a pulled down input pin
                        pub fn into_pull_down_input(
                            self,
                            moder: &mut MODER,
                            pupdr: &mut PUPDR,
                        ) -> $PXi<Input<PullDown>> {
                            let offset = 2 * $i;

                            // input mode
                            moder
                                .moder()
                                .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                            // pull-down
                            pupdr.pupdr().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset))
                            });

                            $PXi { _mode: PhantomData }
                        }

                        /// Configures the pin to operate as a pulled up input pin
                        pub fn into_pull_up_input(
                            self,
                            moder: &mut MODER,
                            pupdr: &mut PUPDR,
                        ) -> $PXi<Input<PullUp>> {
                            let offset = 2 * $i;

                            // input mode
                            moder
                                .moder()
                                .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                            // pull-up
                            pupdr.pupdr().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset))
                            });

                            $PXi { _mode: PhantomData }
                        }

                        /// Configures the pin to operate as an open drain output pin
                        pub fn into_open_drain_output(
                            self,
                            moder: &mut MODER,
                            otyper: &mut OTYPER,
                        ) -> $PXi<Output<OpenDrain>> {
                            let offset = 2 * $i;

                            // general purpose output mode
                            let mode = 0b01;
                            moder.moder().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                            });

                            // open drain output
                            otyper
                                .otyper()
                                .modify(|r, w| unsafe { w.bits(r.bits() | (0b1 << $i)) });

                            $PXi { _mode: PhantomData }
                        }

                        /// Configures the pin to operate as an push pull output pin
                        pub fn into_push_pull_output(
                            self,
                            moder: &mut MODER,
                            otyper: &mut OTYPER,
                        ) -> $PXi<Output<PushPull>> {
                            let offset = 2 * $i;

                            // general purpose output mode
                            let mode = 0b01;
                            moder.moder().modify(|r, w| unsafe {
                                w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                            });

                            // push pull output
                            otyper
                                .otyper()
                                .modify(|r, w| unsafe { w.bits(r.bits() & !(0b1 << $i)) });

                            $PXi { _mode: PhantomData }
                        }
                    }

                    impl $PXi<Output<OpenDrain>> {
                        /// Enables / disables the internal pull up
                        pub fn internal_pull_up(&mut self, pupdr: &mut PUPDR, on: bool) {
                            let offset = 2 * $i;

                            pupdr.pupdr().modify(|r, w| unsafe {
                                w.bits(
                                    (r.bits() & !(0b11 << offset)) | if on {
                                        0b01 << offset
                                    } else {
                                        0
                                    },
                                )
                            });
                        }
                    }

                    impl<MODE> $PXi<Output<MODE>> {
                        /// Erases the pin number from the type
                        ///
                        /// This is useful when you want to collect the pins into an array where you
                        /// need all the elements to have the same type
                        pub fn downgrade(self) -> $PXx<Output<MODE>> {
                            $PXx {
                                i: $i,
                                _mode: self._mode,
                            }
                        }
                    }

                    impl<MODE> $PXi<Input<MODE>> {
                        /// Erases the pin number from the type
                        ///
                        /// This is useful when you want to collect the pins into an array where you
                        /// need all the elements to have the same type
                        pub fn downgrade(self) -> $PXx<Input<MODE>> {
                            $PXx {
                                i: $i,
                                _mode: self._mode,
                            }
                        }
                    }

                    impl<MODE> OutputPin for $PXi<Output<MODE>> {
                        type Error = ();

                        fn set_high(&mut self) -> Result<(), Self::Error> {
                            // NOTE(unsafe) atomic write to a stateless register
                            unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << $i)) }
                            Ok(())
                        }

                        fn set_low(&mut self) -> Result<(), Self::Error> {
                            // NOTE(unsafe) atomic write to a stateless register
                            unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + $i))) }
                            Ok(())
                        }
                    }

                    #[cfg(feature = "unproven")]
                    impl<MODE> InputPin for $PXi<Input<MODE>> {
                        type Error = ();

                        fn is_high(&self) -> Result<bool, Self::Error> {
                            Ok(!self.is_low()?)
                        }

                         fn is_low(&self) -> Result<bool, Self::Error> {
                            // NOTE(unsafe) atomic read with no side effects
                            Ok(unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << $i) == 0 })
                        }
                    }
                )+
            }
        )+
    }
}
