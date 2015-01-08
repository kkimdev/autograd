extern crate test;
extern crate autograd;

#[test]
fn addition() {
    let context: autograd::ContextImpl = autograd::Context::new();
    let x1 = autograd::Context::new_variable(&context, 1.5);
    let x2 = autograd::Context::new_variable(&context, 2.5);

    let y = x1 + x2;
    assert_eq!(y.value, 4.);

    autograd::Context::differentiate(&context, y);
    assert_eq!(autograd::Context::get_derivative(&context, y), 1.);
    assert_eq!(autograd::Context::get_derivative(&context, x1), 1.);
    assert_eq!(autograd::Context::get_derivative(&context, x2), 1.);
}

#[test]
fn multiplication() {
    let context: autograd::ContextImpl = autograd::Context::new();
    let x1 = autograd::Context::new_variable(&context, 1.5);
    let x2 = autograd::Context::new_variable(&context, 2.5);

    let y = x1 * x2;
    assert_eq!(y.value, 3.75);

    autograd::Context::differentiate(&context, y);
    assert_eq!(autograd::Context::get_derivative(&context, y), 1.);
    assert_eq!(autograd::Context::get_derivative(&context, x1), 2.5);
    assert_eq!(autograd::Context::get_derivative(&context, x2), 1.5);
}

// // TODO Make this ideal API work.
// fn ideal_api() {
//     // TODO Do we want to initialize tape size later?
//     let context = autograd::new_context!(f32, 1024);
//     let x1 = context.new_variable(1.5);
//     let x2 = context.new_variable(2.5);
//
//     let y = 2. * x1 * x2;
//     println!("    y  == {}", y.value);
//
//     context.differentiate(y);
//     println!("dy/dy  == {}", context.get_derivative(y));
//     println!("dy/dx1 == {}", context.get_derivative(x1));
//     println!("dy/dx2 == {}", context.get_derivative(x2));
// }
