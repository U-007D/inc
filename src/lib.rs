#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

#[cfg(test)] mod unit_tests;

extern crate num;
use self::num::Num;
use std::ops::{AddAssign, SubAssign};

pub trait Increment<T> {
    fn _inc(&mut self) -> Self;
    fn _inc_by(&mut self, n: T) -> Self;
    fn inc_(&mut self) -> Self;
    fn inc_by_(&mut self, n: T) -> Self;
}

impl<T> Increment<T> for T where T: Num + AddAssign<T> + Copy {
    fn _inc(&mut self) -> Self { self._inc_by(T::one()) }

    fn _inc_by(&mut self, n: T) -> Self {
        *self += n;
        *self
    }

    fn inc_(&mut self) -> T { self.inc_by_(T::one()) }

    fn inc_by_(&mut self, n: T) -> Self {
        let tmp = *self;
        let _ = self._inc_by(n);
        tmp
    }
}

pub trait Decrement<T> {
    fn _dec(&mut self) -> Self;
    fn _dec_by(&mut self, n: T) -> Self;
    fn dec_(&mut self) -> Self;
    fn dec_by_(&mut self, n: T) -> Self;
}

impl<T> Decrement<T> for T where T: Num + SubAssign<T> + Copy {
    fn _dec(&mut self) -> Self { self._dec_by(T::one()) }

    fn _dec_by(&mut self, n: T) -> Self {
        *self -= n;
        *self
    }

    fn dec_(&mut self) -> T { self.dec_by_(T::one()) }

    fn dec_by_(&mut self, n: T) -> Self {
        let tmp = *self;
        let _ = self._dec_by(n);
        tmp
    }
}
