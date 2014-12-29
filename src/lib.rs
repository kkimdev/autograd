/*!
# Autogard
*/

#![crate_name = "autograd"]
#![crate_type = "rlib"]

pub use context::Context;
pub use float::Float;

mod context;
mod float;

#[cfg(test)]
mod tests;
