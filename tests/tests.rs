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

#[test]
fn addition() {
    let context = new_autograd_context!(f32, 1000);
    let x1 = context.new_variable(1.5);
    let x2 = context.new_variable(2.5);

    let y = x1 + x2;
    assert_eq!(y.value, 4.);

    context.differentiate(y);
    assert_eq!(context.get_derivative(x1), 1.);
    assert_eq!(context.get_derivative(x2), 1.);
    assert_eq!(context.get_derivative(y), 1.);
}

#[test]
fn multiplication() {
    let context = new_autograd_context!(f32, 1000);
    let x1 = context.new_variable(1.5);
    let x2 = context.new_variable(2.5);

    let y = x1 * x2;
    assert_eq!(y.value, 3.75);

    context.differentiate(y);
    assert_eq!(context.get_derivative(x1), 2.5);
    assert_eq!(context.get_derivative(x2), 1.5);
    assert_eq!(context.get_derivative(y), 1.);
}

#[test]
fn single_thread_multiple_run() {
    for _ in 0..10 {
        addition();
    }
}

#[test]
fn multi_thread_multiple_run() {
    // TODO Insert purposeful sleep for each task?
    let task_pool = std::sync::TaskPool::new(10);
    let semaphore = std::sync::Arc::new(std::sync::Semaphore::new(-999));
    for _ in 0..1000 {
        let semaphore_task = semaphore.clone();
        task_pool.execute(move || {
            single_thread_multiple_run();
            semaphore_task.release();
        });
    }
    semaphore.access();
}

#[test]
#[should_fail(expected = "This Context instance is in use now. Note that a Context instance is allowed per construction location and per thread. Consequently, it cannot be recursively constructed unless it is destructed. This is a limitation caused by the thread local static variables usages in the current implementation.")]
fn context_lock() {
    fn recursive_context(n: usize) {
        if n > 0 {
            let context = new_autograd_context!(f32, 10);
            recursive_context(n - 1);
        }
    }
    recursive_context(2);
}

#[test]
fn context_capacity_expression_argument() {
    let a = 200;
    let b = 10;
    let context = new_autograd_context!(f32, a/b);
}
