/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*!
# Autogard
*/

#![crate_name = "autograd"]
#![crate_type = "rlib"]
#![feature(thread_local)]

mod context;
mod float;

#[cfg(test)]
mod tests;

pub use context::Context;
// TODO ideally Private traits shouldn't be exported.
pub use context::ContextCratePrivate;
pub use context::ContextModulePrivate;

pub use float::Float;
