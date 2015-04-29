/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use num;
use std;

// TODO derive Hash?
#[derive(Debug)]
pub struct Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    pub value: InternalFloat,
    index: usize,
    phantom_context: std::marker::PhantomData<CT>
}

// TODO implement num::Float
impl <InternalFloat, CT> num::Float for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    fn nan() -> Self {
        unimplemented!();
    }

    fn infinity() -> Self {
        unimplemented!();
    }

    fn neg_infinity() -> Self {
        unimplemented!();
    }

    fn neg_zero() -> Self {
        unimplemented!();
    }

    fn min_value() -> Self {
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

    fn is_sign_positive(self) -> bool {
        self.value.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.value.is_sign_negative()
    }

    #[allow(unused_variables)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        unimplemented!();
    }

    fn recip(self) -> Self {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn powi(self, n: i32) -> Self {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn powf(self, n: Self) -> Self {
        unimplemented!();
    }

    fn sqrt(self) -> Self {
        let two = InternalFloat::one() + InternalFloat::one();
        let sqrt_value = self.value.sqrt();
        Float{value: sqrt_value,
              index: CT::unary_operation((two * sqrt_value).recip(), self.index),
              phantom_context: std::marker::PhantomData}
    }

    fn exp(self) -> Self {
        let exp_value = self.value.exp();
        Float{value: exp_value,
              index: CT::unary_operation(exp_value, self.index),
              phantom_context: std::marker::PhantomData}
    }

    fn exp2(self) -> Self {
        unimplemented!();
    }

    fn ln(self) -> Self {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn log(self, base: Self) -> Self {
        unimplemented!();
    }

    fn log2(self) -> Self {
        unimplemented!();
    }

    fn log10(self) -> Self {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn max(self, other: Self) -> Self {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn min(self, other: Self) -> Self {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn abs_sub(self, other: Self) -> Self {
        unimplemented!();
    }

    fn cbrt(self) -> Self {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn hypot(self, other: Self) -> Self {
        unimplemented!();
    }

    fn sin(self) -> Self {
        Float{value: self.value.sin(),
              index: CT::unary_operation(self.value.cos(), self.index),
              phantom_context: std::marker::PhantomData}
    }

    fn cos(self) -> Self {
        Float{value: self.value.cos(),
              index: CT::unary_operation(-self.value.sin(), self.index),
              phantom_context: std::marker::PhantomData}
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

    #[allow(unused_variables)]
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

    fn integer_decode(self) -> (u64, i16, i8) {
        self.value.integer_decode()
    }
}

impl <InternalFloat, CT> num::Zero for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    fn zero() -> Self {
        unimplemented!();
    }
    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl <InternalFloat, CT> num::One for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    fn one() -> Self {
        unimplemented!();
    }
}

impl <InternalFloat, CT> num::Num for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    type FromStrRadixErr = num::traits::ParseFloatError;

    #[allow(unused_variables)]
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!();
    }
}

impl <InternalFloat, CT> std::clone::Clone for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    fn clone(&self) -> Self {
        Float{value: self.value, index: self.index, phantom_context: std::marker::PhantomData}
    }
}


impl <InternalFloat, CT> num::ToPrimitive for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    fn to_i64(&self) -> Option<i64> { self.value.to_i64() }

    fn to_u64(&self) -> Option<u64> { self.value.to_u64() }
    //TODO implement the rest optional function.
}

impl <InternalFloat, CT> num::NumCast for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    #[allow(unused_variables)]
    fn from<TP>(n: TP) -> Option<Self> where TP: num::ToPrimitive {
        panic!("We disallow constructing from a constant because it is unnecessary.");
    }
}

impl <InternalFloat, CT> std::ops::Neg for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    type Output = Float<InternalFloat, CT>;
    fn neg(self) -> Float<InternalFloat, CT> {
        Float{value: -self.value,
              index: CT::unary_operation(InternalFloat::one().neg(), self.index),
              phantom_context: std::marker::PhantomData}
    }
}

// TODO adjoints of 1 can be optimized out, i.e., not multiplying. Should we?
// TODO implement operations with underlying type.
impl <InternalFloat, CT> std::ops::Add<Float<InternalFloat, CT>> for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    type Output = Float<InternalFloat, CT>;
    fn add(self, other: Float<InternalFloat, CT>) -> Float<InternalFloat, CT> {
        Float{value: self.value + other.value,
              index: CT::binary_operation(
                  &[num::One::one(), num::One::one()],
                  &[self.index, other.index]),
              phantom_context: std::marker::PhantomData}
    }
}

impl <InternalFloat, CT> std::ops::Add<InternalFloat> for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    type Output = Float<InternalFloat, CT>;
    fn add(self, other: InternalFloat) -> Float<InternalFloat, CT> {
        Float{value: self.value + other,
              index: CT::unary_operation(num::One::one(), self.index),
              phantom_context: std::marker::PhantomData}
    }
}

impl <InternalFloat, CT> std::ops::Sub<Float<InternalFloat, CT>> for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    type Output = Float<InternalFloat, CT>;
    fn sub(self, other: Float<InternalFloat, CT>) -> Float<InternalFloat, CT> {
        let one: InternalFloat = num::One::one();
        Float{value: self.value - other.value,
              index: CT::binary_operation(&[one, -one], &[self.index, other.index]),
              phantom_context: std::marker::PhantomData}
    }
}

impl <InternalFloat, CT> std::ops::Sub<InternalFloat> for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    type Output = Float<InternalFloat, CT>;
    fn sub(self, other: InternalFloat) -> Float<InternalFloat, CT> {
        Float{value: self.value - other,
              index: CT::unary_operation(num::One::one(), self.index),
              phantom_context: std::marker::PhantomData}
    }
}

impl <InternalFloat, CT> std::ops::Mul<Float<InternalFloat, CT>> for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    type Output = Float<InternalFloat, CT>;
    fn mul(self, other: Float<InternalFloat, CT>) -> Float<InternalFloat, CT> {
        Float{value: self.value * other.value,
              index: CT::binary_operation(
                  &[other.value, self.value],
                  &[self.index, other.index]),
              phantom_context: std::marker::PhantomData}
    }
}

impl <InternalFloat, CT> std::ops::Mul<InternalFloat> for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    type Output = Float<InternalFloat, CT>;
    fn mul(self, other: InternalFloat) -> Float<InternalFloat, CT> {
        Float{value: self.value * other,
              index: CT::unary_operation(other, self.index),
              phantom_context: std::marker::PhantomData}
    }
}

impl <InternalFloat, CT> std::ops::Div<Float<InternalFloat, CT>> for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    type Output = Float<InternalFloat, CT>;
    fn div(self, other: Float<InternalFloat, CT>) -> Float<InternalFloat, CT> {
        Float{value: self.value / other.value,
              index: CT::binary_operation(
                  &[other.value.recip(), -((self.value * self.value).recip())],
                  &[self.index, other.index]),
              phantom_context: std::marker::PhantomData}
    }
}

impl <InternalFloat, CT> std::ops::Div<InternalFloat> for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    type Output = Float<InternalFloat, CT>;
    fn div(self, other: InternalFloat) -> Float<InternalFloat, CT> {
        Float{value: self.value / other,
              index: CT::unary_operation(other.recip(), self.index),
              phantom_context: std::marker::PhantomData}
    }
}

// TODO 1. Does it make sense to support % operator between two Floats ?
//      2. If so, should we record other even though the multiplier is 0?
impl <InternalFloat, CT> std::ops::Rem<Float<InternalFloat, CT>> for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    type Output = Float<InternalFloat, CT>;
    fn rem(self, other: Float<InternalFloat, CT>) -> Float<InternalFloat, CT> {
        // TODO add this kind of assert everywhere.
        // assert!(( self . context as * const super::context::Context < InternalFloat >
        //         ) == (
        //         other . context as * const super::context::Context < InternalFloat > ));
        Float{value: self.value % other.value,
              index: CT::binary_operation(
                  &[num::One::one(), num::Zero::zero()],
                  &[self.index, other.index]),
              phantom_context: std::marker::PhantomData}
    }
}

impl <InternalFloat, CT> std::ops::Rem<InternalFloat> for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    type Output = Float<InternalFloat, CT>;
    fn rem(self, other: InternalFloat) -> Float<InternalFloat, CT> {
        Float{value: self.value % other,
              index: CT::unary_operation(num::One::one(), self.index),
              phantom_context: std::marker::PhantomData}
    }
}

macro_rules! impl_std_ops {
    ($InternalFloat:ty) => (
        impl <CT> std::ops::Add<Float<$InternalFloat, CT>> for $InternalFloat where CT: super::context::Context<$InternalFloat> {
            type Output = Float<$InternalFloat, CT>;
            fn add(self, other: Float<$InternalFloat, CT>) -> Float<$InternalFloat, CT> {
                other.add(self)
            }
        }

        impl <CT> std::ops::Sub<Float<$InternalFloat, CT>> for $InternalFloat where CT: super::context::Context<$InternalFloat> {
            type Output = Float<$InternalFloat, CT>;
            fn sub(self, other: Float<$InternalFloat, CT>) -> Float<$InternalFloat, CT> {
                Float{value: self - other.value,
                      index: CT::unary_operation(
                          -<$InternalFloat as num::One>::one(), other.index),
                      phantom_context: std::marker::PhantomData}
            }
        }

        impl <CT> std::ops::Mul<Float<$InternalFloat, CT>> for $InternalFloat where CT: super::context::Context<$InternalFloat> {
            type Output = Float<$InternalFloat, CT>;
            fn mul(self, other: Float<$InternalFloat, CT>) -> Float<$InternalFloat, CT> {
                other.mul(self)
            }
        }

        impl <CT> std::ops::Div<Float<$InternalFloat, CT>> for $InternalFloat where CT: super::context::Context<$InternalFloat> {
            type Output = Float<$InternalFloat, CT>;
            fn div(self, other: Float<$InternalFloat, CT>) -> Float<$InternalFloat, CT> {
                Float{value: self / other.value,
                      index: CT::unary_operation(
                          -((other.value * other.value).recip()), other.index),
                      phantom_context: std::marker::PhantomData}
            }
        }

        impl <CT> std::ops::Rem<Float<$InternalFloat, CT>> for $InternalFloat where CT: super::context::Context<$InternalFloat> {
            type Output = Float<$InternalFloat, CT>;
            #[allow(unused_variables)]
            fn rem(self, other: Float<$InternalFloat, CT>) -> Float<$InternalFloat, CT> {
                unimplemented!();
            }
        }
    )
}

// TODO We shouldn't hard code f32 and f64. Or at least, users should be able to implement this for their choice of Float type without modifying this library.
impl_std_ops!(f32);
impl_std_ops!(f64);

impl <InternalFloat, CT> std::cmp::PartialEq for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    fn eq(&self, other: &Float<InternalFloat, CT>) -> bool { self.value == other.value }
}

impl <InternalFloat, CT> std::cmp::PartialOrd for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    fn partial_cmp(&self, other: &Float<InternalFloat, CT>) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl <InternalFloat, CT> std::marker::Copy for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
}

pub trait FloatCratePrivate<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    fn new(value: InternalFloat, index: usize) ->Self;
    fn float_get_index(&self) -> usize;
}

impl <InternalFloat, CT> FloatCratePrivate<InternalFloat, CT> for Float<InternalFloat, CT> where InternalFloat: num::Float, CT: super::context::Context<InternalFloat> {
    fn new(value: InternalFloat, index: usize) -> Self {
        Float{value: value, index: index, phantom_context: std::marker::PhantomData}
    }

    fn float_get_index(&self) -> usize {
        self.index
    }
}
