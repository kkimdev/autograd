/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*!
# Autogard
*/

// TODO Using expression template will give better performance.
//      e.g., http://www.met.reading.ac.uk/clouds/adept/

// TODO Multi-threading support.

// TODO Add Valgrind, ASAN, and TSAN tests.

#![crate_name = "autograd"]
#![crate_type = "rlib"]

#![feature(thread_local)]

extern crate num;

mod context;
mod float;

pub use context::Context;
// TODO ideally Private traits shouldn't be exported.
pub use context::ContextCratePrivate;
pub use context::ContextModulePrivate;

pub use float::Float;
