/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate std;

pub struct Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    pub value: T,
    index: usize,
}

// TODO implement std::num::Float
impl <T, CT> std::num::Float for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    fn nan() -> Self {
        unimplemented!();
    }

    fn infinity() -> Self {
        unimplemented!();
    }

    fn neg_infinity() -> Self {
        unimplemented!();
    }

    fn zero() -> Self {
        unimplemented!();
    }

    fn neg_zero() -> Self {
        unimplemented!();
    }

    fn one() -> Self {
        unimplemented!();
    }

    fn mantissa_digits(unused_self: Option<Self>) -> usize {
        unimplemented!();
    }

    fn digits(unused_self: Option<Self>) -> usize {
        unimplemented!();
    }

    fn epsilon() -> Self {
        unimplemented!();
    }

    fn min_exp(unused_self: Option<Self>) -> isize {
        unimplemented!();
    }

    fn max_exp(unused_self: Option<Self>) -> isize {
        unimplemented!();
    }

    fn min_10_exp(unused_self: Option<Self>) -> isize {
        unimplemented!();
    }

    fn max_10_exp(unused_self: Option<Self>) -> isize {
        unimplemented!();
    }

    fn min_value() -> Self {
        unimplemented!();
    }

    fn min_pos_value(unused_self: Option<Self>) -> Self {
        unimplemented!();
    }

    fn max_value() -> Self {
        unimplemented!();
    }

    fn is_nan(self) -> bool {
        self.value.is_nan()
    }

    fn is_infinite(self) -> bool {
        self.value.is_infinite()
    }

    fn is_finite(self) -> bool {
        self.value.is_finite()
    }

    fn is_normal(self) -> bool {
        self.value.is_normal()
    }

    fn classify(self) -> std::num::FpCategory {
        self.value.classify()
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        self.value.integer_decode()
    }

    fn floor(self) -> Self {
        unimplemented!();
    }

    fn ceil(self) -> Self {
        unimplemented!();
    }

    fn round(self) -> Self {
        unimplemented!();
    }

    fn trunc(self) -> Self {
        unimplemented!();
    }

    fn fract(self) -> Self {
        unimplemented!();
    }

    fn abs(self) -> Self {
        unimplemented!();
    }

    fn signum(self) -> Self {
        unimplemented!();
    }

    fn is_positive(self) -> bool {
        self.value.is_positive()
    }

    fn is_negative(self) -> bool {
        self.value.is_negative()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        unimplemented!();
    }

    fn recip(self) -> Self {
        unimplemented!();
    }

    fn powi(self, n: i32) -> Self {
        unimplemented!();
    }

    fn powf(self, n: Self) -> Self {
        unimplemented!();
    }

    fn sqrt(self) -> Self {
        let one = <T as std::num::Float>::one();
        let two = one + one;
        let sqrt_value = self.value.sqrt();
        Float{value: sqrt_value,
              index: <CT as super::context::ContextCratePrivate<T>>::unary_operation(
                  (two * sqrt_value).recip(), self.index)}
    }

    fn rsqrt(self) -> Self {
        unimplemented!();
    }

    fn exp(self) -> Self {
        let exp_value = self.value.exp();
        Float{value: exp_value,
              index: <CT as super::context::ContextCratePrivate<T>>::unary_operation(
                  exp_value, self.index)}
    }

    fn exp2(self) -> Self {
        unimplemented!();
    }

    fn ln(self) -> Self {
        unimplemented!();
    }

    fn log(self, base: Self) -> Self {
        unimplemented!();
    }

    fn log2(self) -> Self {
        unimplemented!();
    }

    fn log10(self) -> Self {
        unimplemented!();
    }

    fn to_degrees(self) -> Self {
        unimplemented!();
    }

    fn to_radians(self) -> Self {
        unimplemented!();
    }

    fn ldexp(x: Self, exp: isize) -> Self {
        unimplemented!();
    }

    fn frexp(self) -> (Self, isize) {
        unimplemented!();
    }

    fn next_after(self, other: Self) -> Self {
        unimplemented!();
    }

    fn max(self, other: Self) -> Self {
        unimplemented!();
    }

    fn min(self, other: Self) -> Self {
        unimplemented!();
    }

    fn abs_sub(self, other: Self) -> Self {
        unimplemented!();
    }

    fn cbrt(self) -> Self {
        unimplemented!();
    }

    fn hypot(self, other: Self) -> Self {
        unimplemented!();
    }

    fn sin(self) -> Self {
        Float{value: self.value.sin(),
              index: <CT as super::context::ContextCratePrivate<T>>::unary_operation(
                  self.value.cos(), self.index)}
    }

    fn cos(self) -> Self {
        Float{value: self.value.cos(),
              index: <CT as super::context::ContextCratePrivate<T>>::unary_operation(
                  -self.value.sin(), self.index)}
    }

    fn tan(self) -> Self {
        unimplemented!();
    }

    fn asin(self) -> Self {
        unimplemented!();
    }

    fn acos(self) -> Self {
        unimplemented!();
    }

    fn atan(self) -> Self {
        unimplemented!();
    }

    fn atan2(self, other: Self) -> Self {
        unimplemented!();
    }

    fn sin_cos(self) -> (Self, Self) {
        unimplemented!();
    }

    fn exp_m1(self) -> Self {
        unimplemented!();
    }

    fn ln_1p(self) -> Self {
        unimplemented!();
    }

    fn sinh(self) -> Self {
        unimplemented!();
    }

    fn cosh(self) -> Self {
        unimplemented!();
    }

    fn tanh(self) -> Self {
        unimplemented!();
    }

    fn asinh(self) -> Self {
        unimplemented!();
    }

    fn acosh(self) -> Self {
        unimplemented!();
    }

    fn atanh(self) -> Self {
        unimplemented!();
    }
}

impl <T, CT> std::clone::Clone for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    fn clone(&self) -> Self {
        Float{value: self.value, index: self.index}
    }
}


impl <T, CT> std::num::ToPrimitive for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    fn to_i64(&self) -> Option<i64> { self.value.to_i64() }

    fn to_u64(&self) -> Option<u64> { self.value.to_u64() }
    //TODO implement the rest optional function.
}

impl <T, CT> std::num::NumCast for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    fn from<TP>(n: TP) -> Option<Self> where TP: std::num::ToPrimitive {
        None
    }
}

impl <T, CT> std::ops::Neg for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    type Output = Float<T, CT>;
    fn neg(self) -> Float<T, CT> {
        // TODO rust doesn't have T::one() yet.
        let one: T = std::num::Float::one();
        Float{value: -self.value,
              index: <CT as super::context::ContextCratePrivate<T>>::unary_operation(one.neg(), self.index)}
    }
}

// TODO adjoints of 1 can be optimized out, i.e., not multiplying. Should we?
impl <T, CT> std::ops::Add<Float<T, CT>> for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    type Output = Float<T, CT>;
    fn add(self, rhs: Float<T, CT>) -> Float<T, CT> {
        Float{value: self.value + rhs.value,
              index: <CT as super::context::ContextCratePrivate<T>>::binary_operation(
                  &[std::num::Float::one(), std::num::Float::one()],
                  &[self.index, rhs.index])}
    }
}

impl <T, CT> std::ops::Sub<Float<T, CT>> for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    type Output = Float<T, CT>;
    fn sub(self, rhs: Float<T, CT>) -> Float<T, CT> {
        let one: T = std::num::Float::one();
        Float{value: self.value - rhs.value,
              index: <CT as super::context::ContextCratePrivate<T>>::binary_operation(
                  &[one, -one],
                  &[self.index, rhs.index])}
    }
}

impl <T, CT> std::ops::Mul<Float<T, CT>> for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    type Output = Float<T, CT>;
    fn mul(self, rhs: Float<T, CT>) -> Float<T, CT> {
        Float{value: self.value * rhs.value,
              index: <CT as super::context::ContextCratePrivate<T>>::binary_operation(
                  &[rhs.value, self.value],
                  &[self.index, rhs.index])}
    }
}

impl <T, CT> std::ops::Div<Float<T, CT>> for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    type Output = Float<T, CT>;
    fn div(self, rhs: Float<T, CT>) -> Float<T, CT> {
        Float{value: self.value / rhs.value,
              index: <CT as super::context::ContextCratePrivate<T>>::binary_operation(
                  &[rhs.value.recip(), -((self.value * self.value).recip())],
                  &[self.index, rhs.index])}
    }
}

// TODO 1. Does it make sense to support % operator between two Floats ?
//      2. If so, should we record rhs even though the multiplier is 0?
impl <T, CT> std::ops::Rem<Float<T, CT>> for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    type Output = Float<T, CT>;
    fn rem(self, rhs: Float<T, CT>) -> Float<T, CT> {
        // TODO add this kind of assert everywhere.
        // assert!(( self . context as * const super::context::Context < T >
        //         ) == (
        //         rhs . context as * const super::context::Context < T > ));
        Float{value: self.value % rhs.value,
              index: <CT as super::context::ContextCratePrivate<T>>::binary_operation(
                  &[std::num::Float::one(), std::num::Float::zero()],
                  &[self.index, rhs.index])}
    }
}

impl <T, CT> std::cmp::PartialEq for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    fn eq(&self, other: &Float<T, CT>) -> bool { self.value == other.value }
}

impl <T, CT> std::cmp::PartialOrd for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    fn partial_cmp(&self, other: &Float<T, CT>) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl <T, CT> std::marker::Copy for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
}

// Crate private functions

pub trait FloatCratePrivate<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    fn new(value: T, index: usize) ->Self;
    fn float_get_index(&self) -> usize;
}

impl <T, CT> FloatCratePrivate<T, CT> for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    fn new(value: T, index: usize) -> Self {
        Float{value: value, index: index}
    }

    fn float_get_index(&self) -> usize {
        self.index
    }
}
