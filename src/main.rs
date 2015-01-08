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
