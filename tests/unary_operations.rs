/* Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */

#[macro_use]
extern crate autograd;

use autograd::Context;
use std::num::Float;
use std::ops::Neg;

macro_rules! unary_operation_test {
    ($name:ident, $x:expr, $y:expr, $dx:expr) => (
        #[test]
        fn $name() {
            let context = new_autograd_context!(f32, 1000);
            let x = context.new_variable($x);

            let y = x.$name();
            assert_eq!(y.value, $y);

            context.differentiate(y);
            assert_eq!(context.get_derivative(x), $dx);
            assert_eq!(context.get_derivative(y), 1.);
        }
    )
}

unary_operation_test!(cos, 0., 1., 0.);
unary_operation_test!(neg, 1.5, -1.5, -1.);
