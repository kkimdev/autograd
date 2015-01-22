/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// #![feature(thread_local)]
//
// extern crate autograd;
//
// trait Trait: std::marker::Sized {
//
// }
//
// struct Struct;
//
// impl Trait for Struct {
// }
//
// impl std::ops::Drop for Struct {
//     fn drop(&mut self) {
//         println!("dropped");
//     }
// }
//
// fn main() {
//     let s = Struct;
//     let t : Trait = s;
//
// }


fn main() {
    let x = 10;
    for i in (0..x).rev() {
        println!("{}", i);
    }
}
