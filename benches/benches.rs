/* Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */

#![feature(thread_local)]
#![feature(std_misc)]
#![feature(alloc)]
#![feature(test)]
#![feature(convert)]

extern crate num;
extern crate test;
#[macro_use]
extern crate autograd;

static BENCH_SIZE : usize = 1024;

fn norm<T>(inputs: &[T]) -> T where T : num::Float {
    let mut inputs_iter = inputs.iter();
    let first_input = *inputs_iter.next().unwrap();
    let mut sum = first_input * first_input;
    for &input in inputs_iter {
        sum = sum + (input * input);
    }
    sum.sqrt()
}

#[bench]
fn compute_norm_f32(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        use num::Float;

        let mut x = Vec::<f32>::with_capacity(8 * BENCH_SIZE);

        for _ in (0..BENCH_SIZE) {
            x.push(2.0);
        }

        let y = norm(x.as_slice());
        assert_eq!(y, 2.0 * (BENCH_SIZE as f32).sqrt());
        y
    });
}

#[bench]
fn compute_norm_autograd_f32(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        use num::Float;
        use autograd::Context;

        let context = new_autograd_context!(f32, 8 * BENCH_SIZE);
        let mut x = Vec::with_capacity(BENCH_SIZE);

        for _ in (0..BENCH_SIZE) {
            x.push(context.new_variable(2.0));
        }

        let y = norm(x.as_slice());
        assert_eq!(y.value, 2.0 * (BENCH_SIZE as f32).sqrt());

        context.differentiate(y);
        context.get_derivative(x[0])
    });
}
