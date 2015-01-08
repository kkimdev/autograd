
//impl<T : Float> std::num::FloatMath for AutogradFloat<T> {
//}

// impl <'a, T: Float> Float for AutogradFloat<'a, T> {
//     fn nan() -> AutogradFloat<'a, T> { AutogradFloat{x: Float::nan(), tape: None, tape_index: 0,} }
//
//     fn infinity() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::infinity(), tape: None, tape_index: 0,}
//     }
//
//     fn neg_infinity() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::neg_infinity(), tape: None, tape_index: 0,}
//     }
//
//     fn zero() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::zero(), tape: None, tape_index: 0,}
//     }
//
//     fn neg_zero() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::neg_zero(), tape: None, tape_index: 0,}
//     }
//
//     fn one() -> AutogradFloat<'a, T> { AutogradFloat{x: Float::one(), tape: None, tape_index: 0,} }
//
//     fn is_nan(self) -> bool { self.x.is_nan() }
//
//     fn is_infinite(self) -> bool { self.x.is_infinite() }
//
//     fn is_finite(self) -> bool { self.x.is_finite() }
//
//     fn is_normal(self) -> bool { self.x.is_normal() }
//
//     fn classify(self) -> std::num::FPCategory { self.x.classify() }
//
//     fn mantissa_digits(unused_self: Option<AutogradFloat<T>>) -> uint { 000 }
//
//     fn digits(unused_self: Option<AutogradFloat<T>>) -> uint { 000 }
//
//     fn epsilon() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::epsilon(), tape: None, tape_index: 0,}
//     }
//
//     fn min_exp(unused_self: Option<AutogradFloat<T>>) -> int { 000 }
//
//     fn max_exp(unused_self: Option<AutogradFloat<T>>) -> int { 000 }
//
//     fn min_10_exp(unused_self: Option<AutogradFloat<T>>) -> int { 000 }
//
//     fn max_10_exp(unused_self: Option<AutogradFloat<T>>) -> int { 000 }
//
//     fn min_value() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::min_value(), tape: None, tape_index: 0,}
//     }
//
//     fn min_pos_value(unused_self: Option<AutogradFloat<T>>) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::epsilon(), tape: None, tape_index: 0,} // 000
//     }
//
//     fn max_value() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::max_value(), tape: None, tape_index: 0,} // 000
//     }
//
//     fn integer_decode(self) -> (u64, i16, i8) { self.x.integer_decode() }
//
//     fn floor(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.floor(), tape: None, tape_index: 0,}
//     }
//
//     fn ceil(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.ceil(), tape: None, tape_index: 0,}
//     }
//
//     fn round(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.round(), tape: None, tape_index: 0,}
//     }
//
//     fn trunc(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.trunc(), tape: None, tape_index: 0,}
//     }
//
//     fn fract(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.fract(), tape: None, tape_index: 0,}
//     }
//
//     fn abs(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.abs(), tape: None, tape_index: 0,}
//     }
//
//     fn signum(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.signum(), tape: None, tape_index: 0,}
//     }
//
//     fn is_positive(self) -> bool { self.x.is_positive() }
//
//     fn is_negative(self) -> bool { self.x.is_negative() }
//
//     fn mul_add(self, a: AutogradFloat<T>, b: AutogradFloat<T>) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.mul_add(a.x, b.x), tape: None, tape_index: 0,}
//     }
//
//     fn recip(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.recip(), tape: None, tape_index: 0,}
//     }
//
//     fn powi(self, n: i32) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.powi(n), tape: None, tape_index: 0,}
//     }
//
//     fn powf(self, n: AutogradFloat<T>) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.powf(n.x), tape: None, tape_index: 0,}
//     }
//
//     fn sqrt2() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::sqrt2(), tape: None, tape_index: 0,}
//     }
//
//     fn frac_1_sqrt2() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::frac_1_sqrt2(), tape: None, tape_index: 0,}
//     }
//
//     fn sqrt(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.sqrt(), tape: None, tape_index: 0,}
//     }
//
//     fn rsqrt(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.rsqrt(), tape: None, tape_index: 0,}
//     }
//
//     fn pi() -> AutogradFloat<'a, T> { AutogradFloat{x: Float::pi(), tape: None, tape_index: 0,} }
//
//     fn two_pi() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::two_pi(), tape: None, tape_index: 0,}
//     }
//
//     fn frac_pi_2() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::frac_pi_2(), tape: None, tape_index: 0,}
//     }
//
//     fn frac_pi_3() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::frac_pi_3(), tape: None, tape_index: 0,}
//     }
//
//     fn frac_pi_4() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::frac_pi_4(), tape: None, tape_index: 0,}
//     }
//
//     fn frac_pi_6() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::frac_pi_6(), tape: None, tape_index: 0,}
//     }
//
//     fn frac_pi_8() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::frac_pi_8(), tape: None, tape_index: 0,}
//     }
//
//     fn frac_1_pi() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::frac_1_pi(), tape: None, tape_index: 0,}
//     }
//
//     fn frac_2_pi() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::frac_2_pi(), tape: None, tape_index: 0,}
//     }
//
//     fn frac_2_sqrtpi() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::frac_2_sqrtpi(), tape: None, tape_index: 0,}
//     }
//
//     fn e() -> AutogradFloat<'a, T> { AutogradFloat{x: Float::e(), tape: None, tape_index: 0,} }
//
//     fn log2_e() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::log2_e(), tape: None, tape_index: 0,}
//     }
//
//     fn log10_e() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::log10_e(), tape: None, tape_index: 0,}
//     }
//
//     fn ln_2() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::ln_2(), tape: None, tape_index: 0,}
//     }
//
//     fn ln_10() -> AutogradFloat<'a, T> {
//         AutogradFloat{x: Float::ln_10(), tape: None, tape_index: 0,}
//     }
//
//     fn exp(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.exp(), tape: None, tape_index: 0,}
//     }
//
//     fn exp2(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.exp2(), tape: None, tape_index: 0,}
//     }
//
//     fn ln(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.ln(), tape: None, tape_index: 0,}
//     }
//
//     fn log(self, base: AutogradFloat<T>) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.log(base.x), tape: None, tape_index: 0,}
//     }
//
//     fn log2(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.log2(), tape: None, tape_index: 0,}
//     }
//
//     fn log10(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.log10(), tape: None, tape_index: 0,}
//     }
//
//     fn to_degrees(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.to_degrees(), tape: None, tape_index: 0,}
//     }
//
//     fn to_radians(self) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.to_radians(), tape: None, tape_index: 0,}
//     }
//
// }

// TODO : Maybe we shouldn't allow converting to AutogradFloat type from a raw number.
//        because ideally we shouldn't have AutogradFloat type constant.
//        Or maybe just return None always.
// impl <'a, T: Float> std::num::NumCast for AutogradFloat<'a, T> {
//     fn from<U: ToPrimitive>(n: U) -> Option<AutogradFloat<'a, T>> {
//         match std::num::NumCast::from(n) {
//             Some(x) => Some(AutogradFloat::<T>{x: x, tape: None, tape_index: 0,}),
//             None => None,
//         }
//     }
// }


// TODO Float trait's Binary operators' rhs are value. Should we get a reference?
// impl <'a, T: Float> std::ops::Add<&'a AutogradFloat<'a, T>, AutogradFloat<'a, T>> for AutogradFloat<'a, T> {
//     fn add(self, rhs: &AutogradFloat<T>) -> AutogradFloat<'a T> {
//          AutogradFloat{x: self.x.add(rhs.x), tape: None, tape_index: 0,}
//     }
// }

// TODO implement operations with underlying type.
// impl <'a, T: Float> std::ops::Add<T, AutogradFloat<'a, T>> for AutogradFloat<'a, T> {
//     fn add(&self, rhs: &T) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: self.x.add(rhs), tape: None, tape_index: 0,}
//     }
// }
//
//
// impl <'a, T: Float> std::ops::Add<AutogradFloat<'a, T>, AutogradFloat<'a, T>> for T {
//     fn add(&self, rhs: &AutogradFloat<'a, T>) -> AutogradFloat<'a, T> {
//         AutogradFloat{x: rhs.x.add(self), tape: None, tape_index: 0,}
//     }
// }


    // impl <T: std::num::Float> Tape<T> {
    //     fn new(size: usize) -> Tape<T> {
    //         Tape{recorded_variables_count: 0,
    //              adjoints: Vec::with_capacity(size),
    //              lhs_indices: Vec::with_capacity(size),
    //              rhs_indices: Vec::with_capacity(size),
    //              result_derivatives: None,}
    //     }
