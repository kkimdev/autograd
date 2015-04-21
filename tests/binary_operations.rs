/* Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */

#![feature(thread_local)]
#![feature(std_misc)]
#![feature(alloc)]

#[macro_use]
extern crate autograd;

use autograd::Context;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;

// TODO Write binary operation tests with constants involved.
//      There is a Rust bug regarding this though. https://github.com/rust-lang/rust/issues/19035
macro_rules! binary_operation_test {
    ($name:ident, $x1:expr, $x2:expr, $y:expr, $dx1:expr, $dx2:expr) => (
        #[test]
        fn $name() {
            let context = new_autograd_context!(f32, 1000);
            let x1 = context.new_variable($x1);
            let x2 = context.new_variable($x2);

            let y = x1.$name(x2);
            assert_eq!(y.value, $y);

            context.differentiate(y);
            assert_eq!(context.get_derivative(x1), $dx1);
            assert_eq!(context.get_derivative(x2), $dx2);
            assert_eq!(context.get_derivative(y), 1.);
        }
    )
}

binary_operation_test!(add, 1.5, 2.5, 4., 1., 1.);
binary_operation_test!(mul, 1.5, 2.5, 3.75, 2.5, 1.5);
binary_operation_test!(div, 2.0, 1.0, 2.0, 1.0, -0.25);
