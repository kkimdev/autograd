extern crate test;
extern crate autograd;

#[test]
fn addition() {
    let context = autograd::Context::<f32>::new();
    let x1 = context.new_variable(1.5);
    let x2 = context.new_variable(2.5);

    let y = x1 + x2;
    assert_eq!(y.get_value(), 4.);

    context.differentiate(y);
    assert_eq!(context.get_derivative(x1), 1.);
    assert_eq!(context.get_derivative(x2), 1.);
}

#[test]
fn multiplication() {
    let context = autograd::Context::<f32>::new();
    let x1 = context.new_variable(1.5);
    let x2 = context.new_variable(2.5);

    let y = x1 * x2;
    assert_eq!(y.get_value(), 3.75);

    context.differentiate(y);
    assert_eq!(context.get_derivative(x1), 2.5);
    assert_eq!(context.get_derivative(x2), 1.5);
}
