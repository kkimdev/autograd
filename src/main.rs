/* Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */

#[macro_use]
extern crate autograd;
use autograd::Context;

fn main() {
    let context = new_autograd_context!(f32, 1000);
    let x1 = context.new_variable(1.5);
}
