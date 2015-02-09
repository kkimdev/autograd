/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate std;

#[derive(Clone)]
pub struct Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    pub value: T,
    index: usize,
}

// TODO Remove this. as value bing public, we don't need anymore.
// impl <T: std::num::Float, CT: super::context::Context<T>> Float<T, CT> {
//     pub fn get_value(&self) -> T {
//         self.value
//     }
// }

// TODO implement std::num::Float
// impl <T: std::num::Float, CT: super::context::Context<T>> std::num::Float for Float<T, CT> {
// }

impl <T, CT> std::num::ToPrimitive for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    fn to_i64(&self) -> Option<i64> { self.value.to_i64() }

    fn to_u64(&self) -> Option<u64> { self.value.to_u64() }
    //TODO implement the rest optional function.
}

// TODO implement std::num::NumCast, actually, can I? Should we allow?
// impl <T: std::num::Float, CT: super::context::Context<T>> std::num::NumCast for Float<T, CT> {
// }

impl <T, CT> std::ops::Neg for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    type Output = Float<T, CT>;
    fn neg(self) -> Float<T, CT> {
        // TODO rust doesn't have T::one() yet.
        let one: T = std::num::Float::one();
        Float{value: -self.value,
              index: <CT as super::context::Context<T>>::unary_operation(one.neg(), self.index)}
    }
}

// TODO adjoints of 1 can be optimized out, i.e., not multiplying. Should we?
impl <T, CT> std::ops::Add<Float<T, CT>> for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    type Output = Float<T, CT>;
    fn add(self, rhs: Float<T, CT>) -> Float<T, CT> {
        Float{value: self.value + rhs.value,
              index: <CT as super::context::Context<T>>::binary_operation(
                  &[std::num::Float::one(), std::num::Float::one()],
                  &[self.index, rhs.index])}
    }
}

impl <T, CT> std::ops::Sub<Float<T, CT>> for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    type Output = Float<T, CT>;
    fn sub(self, rhs: Float<T, CT>) -> Float<T, CT> {
        let one: T = std::num::Float::one();
        Float{value: self.value - rhs.value,
              index: <CT as super::context::Context<T>>::binary_operation(
                  &[one, -one],
                  &[self.index, rhs.index])}
    }
}

impl <T, CT> std::ops::Mul<Float<T, CT>> for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    type Output = Float<T, CT>;
    fn mul(self, rhs: Float<T, CT>) -> Float<T, CT> {
        Float{value: self.value * rhs.value,
              index: <CT as super::context::Context<T>>::binary_operation(
                  &[rhs.value, self.value],
                  &[self.index, rhs.index])}
    }
}

impl <T, CT> std::ops::Div<Float<T, CT>> for Float<T, CT> where T: std::num::Float, CT: super::context::Context<T> {
    type Output = Float<T, CT>;
    fn div(self, rhs: Float<T, CT>) -> Float<T, CT> {
        Float{value: self.value / rhs.value,
              index: <CT as super::context::Context<T>>::binary_operation(
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
              index: <CT as super::context::Context<T>>::binary_operation(
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
