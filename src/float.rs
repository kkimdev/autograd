extern crate std;

#[derive(Clone)]
pub struct Float<'a, T: 'a> {
    value: T,
    index: usize,
    context: &'a super::context::Context<T>,
}

impl <'a, T: std::num::Float> Float<'a, T> {
    // TODO Should we return reference or value?
    pub fn get_value(&self) -> T {
        self.value
    }
}

impl <'a, T: std::num::Float> std::num::ToPrimitive for Float<'a, T> {
    fn to_i64(&self) -> Option<i64> { self.value.to_i64() }

    fn to_u64(&self) -> Option<u64> { self.value.to_u64() }
    //TODO implement the rest optional function.
}

impl <'a, T: std::num::Float> std::ops::Neg for Float<'a, T> {
    type Output = Float<'a, T>;
    fn neg(self) -> Float<'a, T> {
        // TODO rust doesn't have T::one() yet.
        let one: T = std::num::Float::one();
        Float{value: -self.value,
              context: self.context,
              index:
                  super::context::unary_operation(self.context, one.neg(),
                                                  self.index),}
    }
}

// TODO adjoints of 1 can be optimized out, i.e., not multiplying. Should we?
impl <'a, T: std::num::Float> std::ops::Add<Float<'a, T>> for
 Float<'a, T> {
    type Output = Float<'a, T>;
    fn add(self, rhs: Float<'a, T>) -> Float<T> {
        Float{value: self.value + rhs.value,
              context: self.context,
              index:
                  super::context::binary_operation(self.context,
                                                   &[std::num::Float::one(),
                                                     std::num::Float::one()],
                                                   &[self.index,
                                                     rhs.index]),}
    }
}

impl <'a, T: std::num::Float> std::ops::Sub<Float<'a, T>> for
 Float<'a, T> {
    type Output = Float<'a, T>;
    fn sub(self, rhs: Float<'a, T>) -> Float<T> {
        let one: T = std::num::Float::one();
        Float{value: self.value - rhs.value,
              context: self.context,
              index:
                  super::context::binary_operation(self.context, &[one, -one],
                                                   &[self.index,
                                                     rhs.index]),}
    }
}

impl <'a, T: std::num::Float> std::ops::Mul<Float<'a, T>> for
 Float<'a, T> {
    type Output = Float<'a, T>;
    fn mul(self, rhs: Float<'a, T>) -> Float<T> {
        Float{value: self.value * rhs.value,
              context: self.context,
              index:
                  super::context::binary_operation(self.context,
                                                   &[rhs.value, self.value],
                                                   &[self.index,
                                                     rhs.index]),}
    }
}

impl <'a, T: std::num::Float> std::ops::Div<Float<'a, T>> for
 Float<'a, T> {
    type Output = Float<'a, T>;
    fn div(self, rhs: Float<'a, T>) -> Float<T> {
        Float{value: self.value / rhs.value,
              context: self.context,
              index:
                  super::context::binary_operation(self.context,
                                                   &[rhs.value.recip(),
                                                     -((self.value *
                                                            self.value).recip())],
                                                   &[self.index,
                                                     rhs.index]),}
    }
}

// TODO 1. Does it make sense to support % operator between two Floats ?
//      2. If so, should we record rhs even though the multiplier is 0?
impl <'a, T: std::num::Float> std::ops::Rem<Float<'a, T>> for
 Float<'a, T> {
    type Output = Float<'a, T>;
    fn rem(self, rhs: Float<'a, T>) -> Float<T> {
        // TODO add this kind of assert everywhere.
        assert!(( self . context as * const super::context::Context < T >
                ) == (
                rhs . context as * const super::context::Context < T > ));
        Float{value: self.value % rhs.value,
              context: self.context,
              index:
                  super::context::binary_operation(self.context,
                                                   &[std::num::Float::one(),
                                                     std::num::Float::zero()],
                                                   &[self.index,
                                                     rhs.index]),}
    }
}

impl <'a, T: std::num::Float> std::cmp::PartialEq for Float<'a, T> {
    fn eq(&self, other: &Float<T>) -> bool { self.value == other.value }
}

impl <'a, T: std::num::Float> std::cmp::PartialOrd for Float<'a, T> {
    fn partial_cmp(&self, other: &Float<T>) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl <'a, T: std::num::Float> std::marker::Copy for Float<'a, T> {}

pub fn float_new<T: std::num::Float>(value: T,
                                     context: &super::context::Context<T>,
                                     index: usize) -> Float<T> {
    Float{value: value, context: context, index: index,}
}

pub fn float_get_index<T: std::num::Float>(float: &Float<T>) -> usize {
    float.index
}
