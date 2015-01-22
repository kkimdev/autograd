# Warning
This library is under development and not ready to use yet.

# Autograd
Rust automatic differentiation library computing gradient values. It is mainly for nonlinear optimization and machine learning.

## Example
~~~rust
fn main() {
    let context = autograd::Context::<f32>::new();

    // Initializes variables to begin with.
    let x1 = context.new_variable(1.5);
    let x2 = context.new_variable(2.5);

    // Computes a math expression.
    let y = x1 + x2;
    println!("y   == {}", y.get_value());

    // Computes gradient with respect to y.
    context.differentiate(y);
    println!("dx1 == {}", context.get_derivative(x1));
    println!("dx2 == {}", context.get_derivative(x2));
}
~~~

~~~rust
fn main() {
    let context = autograd::Context::<f32>::new();

    // Initializes variables to begin with.
    let x1 = autograd::Float::new(context, 1.5);
    let x2 = autograd::Float::new(context, 2.5);

    // Computes a simple math expression.
    let y = x1 + x2;
    println!("y   == {}", y.get_value());

    // Computes gradient with respect to y.
    y.differentiate();
    println!("dx1 == {}", x1.get_derivative());
    println!("dx2 == {}", x2.get_derivative());
}
~~~
