/* Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */

#![feature(thread_local)]
#![feature(std_misc)]
#![feature(alloc)]

#[macro_use]
extern crate autograd;

use autograd::Context;

fn main() {
    // Initialize Autograd context with type f32 and capacity 100.
    let context = new_autograd_context!(f32, 100);

    // Initializes input variables.
    let x1 = context.new_variable(1.5);
    let x2 = context.new_variable(2.0);

    // Computes a math expression.
    let y = (x1 * x2) + x1 + 5.0;
    println!("y   == {}", y.value);

    // Computes gradient with respect to y.
    context.differentiate(y);
    println!("dx1 == {}", context.get_derivative(x1));
    println!("dx2 == {}", context.get_derivative(x2));
}

/* Output
y   == 9.5
dx1 == 3
dx2 == 1.5
*/
