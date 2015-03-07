# Autograd [![Build Status](https://travis-ci.org/kkimdev/autograd.svg?branch=master)](https://travis-ci.org/kkimdev/autograd)
Rust automatic differentiation library to compute gradient values. It is mainly for nonlinear optimization and machine learning. **It is alpha stage yet.**

## Example
~~~rust
#[macro_use]
extern crate autograd;

use autograd::Context;

fn main() {
    // Initialize Autograd context with type f32 and capacity 100.
    let context = new_autograd_context!(f32, 100);

    // Initializes input variables.
    let x1 = context.new_variable(1.5);
    println!("x1  == {}", x1.value); // 1.5
    let x2 = context.new_variable(2.0);
    println!("x2  == {}", x2.value); // 2

    // Computes a math expression.
    let y = (x1 * x2) + x1 + 5.0;
    println!("y = (x1 * x2) + x1 + 5.0");
    println!("y   == {}", y.value); // 9.5

    // Computes gradient with respect to y.
    context.differentiate(y);
    println!("dx1 == {}", context.get_derivative(x1)); // 3
    println!("dx2 == {}", context.get_derivative(x2)); // 1.5
}

~~~
