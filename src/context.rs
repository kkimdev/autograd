/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// TODO Setup contributor license agreement.

//! this is context class to use autograd.

extern crate std;

use super::float::FloatCratePrivate;

// TODO #[inline] where appropriate.

pub trait Context<InternalFloat>: ContextCratePrivate<InternalFloat> + std::marker::Sized where InternalFloat: std::num::Float {
    // public functions

    fn new_variable(&self, value: InternalFloat) -> super::float::Float<InternalFloat, Self> {
        FloatCratePrivate::new(value, <Self as ContextModulePrivate<InternalFloat>>::get_new_variable_index())
    }

    fn differentiate(&self, float: super::float::Float<InternalFloat, Self>) {
        // TODO The current implementation is not performant and dirty.
        unsafe {
            assert!(*<Self as ContextModulePrivate<InternalFloat>>::get_recorded_variables_count() <= self.capacity(),
                "There are more recorded variables, {}, than its capacity, {}. Memory is corrupted. Please consider using bigger capacity.",
                *<Self as ContextModulePrivate<InternalFloat>>::get_recorded_variables_count(), self.capacity());

            for i in (0..(*<Self as ContextModulePrivate<InternalFloat>>::get_recorded_variables_count())) {
                *<Self as ContextModulePrivate<InternalFloat>>::get_result_derivatives().offset(i as isize) = std::num::Float::zero();
            }

            *<Self as ContextModulePrivate<InternalFloat>>::get_result_derivatives().offset(float.float_get_index() as isize) = std::num::Float::one();
            for i in (0..(*<Self as ContextModulePrivate<InternalFloat>>::get_recorded_entries_count())).rev() {
                let lhs_index = *<Self as ContextModulePrivate<InternalFloat>>::get_lhs_indices().offset(i as isize);
                let rhs_index = *<Self as ContextModulePrivate<InternalFloat>>::get_rhs_indices().offset(i as isize);
                *<Self as ContextModulePrivate<InternalFloat>>::get_result_derivatives().offset(rhs_index as isize) =
                    *<Self as ContextModulePrivate<InternalFloat>>::get_result_derivatives().offset(rhs_index as isize)
                    + (*<Self as ContextModulePrivate<InternalFloat>>::get_result_derivatives().offset(lhs_index as isize)
                       * *<Self as ContextModulePrivate<InternalFloat>>::get_adjoints().offset(i as isize));
            }
        }
    }

    fn get_derivative(&self, float: super::float::Float<InternalFloat, Self>) -> InternalFloat {
        let float_index_offset = float.float_get_index() as isize;
        unsafe {
            *<Self as ContextModulePrivate<InternalFloat>>::get_result_derivatives().offset(float_index_offset)
        }
    }
}

pub trait ContextCratePrivate<InternalFloat>: ContextModulePrivate<InternalFloat> where InternalFloat: std::num::Float {
    fn unary_operation(adjoint: InternalFloat, rhs_index: usize) -> usize {
        let lhs_index = <Self as ContextModulePrivate<InternalFloat>>::get_new_variable_index();
        let recorded_entries_count_offset = <Self as ContextModulePrivate<InternalFloat>>::get_new_entry_index() as isize;
        unsafe {
            *<Self as ContextModulePrivate<InternalFloat>>::get_adjoints().offset(recorded_entries_count_offset) = adjoint;
            *<Self as ContextModulePrivate<InternalFloat>>::get_lhs_indices().offset(recorded_entries_count_offset) = lhs_index;
            *<Self as ContextModulePrivate<InternalFloat>>::get_rhs_indices().offset(recorded_entries_count_offset) = rhs_index;
        }
        lhs_index
    }

    fn binary_operation(adjoints: &[InternalFloat; 2],
                        rhs_indices: &[usize; 2]) -> usize {
        let lhs_index = <Self as ContextModulePrivate<InternalFloat>>::get_new_variable_index();
        let recorded_entries_count_offset_1 = <Self as ContextModulePrivate<InternalFloat>>::get_new_entry_index() as isize;
        let recorded_entries_count_offset_2 = <Self as ContextModulePrivate<InternalFloat>>::get_new_entry_index() as isize;

        unsafe {
            // TODO is indexing inefficient?
            *<Self as ContextModulePrivate<InternalFloat>>::get_adjoints().offset(recorded_entries_count_offset_1) = adjoints[0];
            *<Self as ContextModulePrivate<InternalFloat>>::get_lhs_indices().offset(recorded_entries_count_offset_1) = lhs_index;
            *<Self as ContextModulePrivate<InternalFloat>>::get_rhs_indices().offset(recorded_entries_count_offset_1) = rhs_indices[0];

            *<Self as ContextModulePrivate<InternalFloat>>::get_adjoints().offset(recorded_entries_count_offset_2) = adjoints[1];
            *<Self as ContextModulePrivate<InternalFloat>>::get_lhs_indices().offset(recorded_entries_count_offset_2) = lhs_index;
            *<Self as ContextModulePrivate<InternalFloat>>::get_rhs_indices().offset(recorded_entries_count_offset_2) = rhs_indices[1];
        }
        lhs_index
    }
}

pub trait ContextModulePrivate<InternalFloat> where InternalFloat: std::num::Float {
    fn get_new_variable_index() -> usize {
        let count = <Self as ContextModulePrivate<InternalFloat>>::get_recorded_variables_count();
        let index = *count;
        *count += 1;
        index
    }

    fn get_new_entry_index() -> usize {
        let count = <Self as ContextModulePrivate<InternalFloat>>::get_recorded_entries_count();
        let index = *count;
        *count += 1;
        index
    }

    fn capacity(&self) -> usize;

    fn get_recorded_variables_count() -> &'static mut usize;
    fn get_recorded_entries_count() -> &'static mut usize;
    // TODO use 'static lifetime instead?
    fn get_adjoints<'a>() -> &'a mut*mut InternalFloat;
    fn get_lhs_indices<'a>() -> &'a mut*mut usize;
    fn get_rhs_indices<'a>() -> &'a mut*mut usize;
    fn get_result_derivatives<'a>() -> &'a mut*mut InternalFloat;
}

#[macro_export]
macro_rules! new_autograd_context {
    ($InternalFloat:ty, $capacity:expr) => (
        {
            struct ContextImpl {
                capacity: usize,
                #[allow(dead_code)] // mutex_guard is active during its scope. No need to use directly.
                mutex_guard: std::sync::MutexGuard<'static ()>,
            }

            impl $crate::Context<$InternalFloat> for ContextImpl {
            }

            impl $crate::ContextCratePrivate<$InternalFloat> for ContextImpl {
            }

            impl $crate::ContextModulePrivate<$InternalFloat> for ContextImpl {
                fn capacity(&self) -> usize {
                    self.capacity
                }

                fn get_recorded_variables_count() -> &'static mut usize {
                    #[thread_local]
                    static mut ptr : usize = 0;
                    unsafe {
                        &mut ptr
                    }
                }
                fn get_recorded_entries_count() -> &'static mut usize {
                    #[thread_local]
                    static mut ptr : usize = 0;
                    unsafe {
                        &mut ptr
                    }
                }
                fn get_adjoints<'a>() -> &'a mut*mut $InternalFloat {
                    #[thread_local]
                    static mut ptr : *mut $InternalFloat = 0 as *mut $InternalFloat;
                    unsafe {
                        &mut ptr
                    }
                }
                fn get_lhs_indices<'a>() -> &'a mut*mut usize {
                    #[thread_local]
                    static mut ptr : *mut usize = 0 as *mut usize;
                    unsafe {
                        &mut ptr
                    }
                }
                fn get_rhs_indices<'a>() -> &'a mut*mut usize{
                    #[thread_local]
                    static mut ptr : *mut usize = 0 as *mut usize;
                    unsafe {
                        &mut ptr
                    }
                }

                fn get_result_derivatives<'a>() -> &'a mut*mut $InternalFloat {
                    #[thread_local]
                    static mut ptr : *mut $InternalFloat = 0 as *mut $InternalFloat;
                    unsafe {
                        &mut ptr
                    }
                }
            }

            impl ContextImpl {
                fn new(capacity: usize) -> Self {
                    let context;

                    #[thread_local]
                    static LOCK : std::sync::StaticMutex = std::sync::MUTEX_INIT;
                    match LOCK.try_lock() {
                        Ok(guard) => context = ContextImpl{capacity: capacity, mutex_guard: guard},
                        Err(std::sync::TryLockError::WouldBlock) => panic!("This Context instance is in use now. Note that a Context instance is allowed per construction location and per thread. Consequently, it cannot be recursively constructed unless it is destructed. This is a limitation caused by the thread local static variables usages in the current implementation."),
                        Err(std::sync::TryLockError::Poisoned(poison_error)) => panic!("{:?}", poison_error),
                    }

                    // TODO use checked_mul?
                    //      example : let usize_size = capacity.checked_mul(std::mem::size_of::<usize>()).expect("capacity overflow");
                    let usize_size = capacity * std::mem::size_of::<usize>();
                    let t_size = capacity * std::mem::size_of::<$InternalFloat>();
                    // std::rt::heap::allocate(t_size, mem::min_align_of::<InternalFloat>())

                    unsafe {
                        *<Self as $crate::ContextModulePrivate<$InternalFloat>>::get_recorded_variables_count() = 0;
                        *<Self as $crate::ContextModulePrivate<$InternalFloat>>::get_recorded_entries_count() = 0;
                        *<Self as $crate::ContextModulePrivate<$InternalFloat>>::get_adjoints() = std::rt::heap::allocate(t_size, std::mem::align_of::<$InternalFloat>()) as *mut $InternalFloat;
                        *<Self as $crate::ContextModulePrivate<$InternalFloat>>::get_lhs_indices() = std::rt::heap::allocate(usize_size, std::mem::align_of::<usize>()) as *mut usize;
                        *<Self as $crate::ContextModulePrivate<$InternalFloat>>::get_rhs_indices() = std::rt::heap::allocate(usize_size, std::mem::align_of::<usize>()) as *mut usize;
                        // TODO we don't have to allocate get_result_derivatives now, isn't it?
                        *<Self as $crate::ContextModulePrivate<$InternalFloat>>::get_result_derivatives() = std::rt::heap::allocate(t_size, std::mem::align_of::<$InternalFloat>()) as *mut $InternalFloat;
                    }
                    context
                }
            }

            impl std::ops::Drop for ContextImpl {
                fn drop(&mut self) {
                    // *Context::<$InternalFloat>::get_recorded_variables_count(None::<Self>) = 0;
                    // *Context::<$InternalFloat>::get_adjoints(None::<Self>) = 0;
                    // TODO implement.

                    let usize_size = self.capacity * std::mem::size_of::<usize>();
                    let t_size = self.capacity * std::mem::size_of::<$InternalFloat>();

                    unsafe {
                        std::rt::heap::deallocate(*<Self as $crate::ContextModulePrivate<$InternalFloat>>::get_adjoints() as *mut u8, t_size, std::mem::align_of::<$InternalFloat>());
                        std::rt::heap::deallocate(*<Self as $crate::ContextModulePrivate<$InternalFloat>>::get_lhs_indices() as *mut u8, usize_size, std::mem::align_of::<usize>());
                        std::rt::heap::deallocate(*<Self as $crate::ContextModulePrivate<$InternalFloat>>::get_rhs_indices() as *mut u8, usize_size, std::mem::align_of::<usize>());
                        std::rt::heap::deallocate(*<Self as $crate::ContextModulePrivate<$InternalFloat>>::get_result_derivatives() as *mut u8, t_size, std::mem::align_of::<$InternalFloat>());
                    }
                }
            }

            ContextImpl::new($capacity)
        }
    )
}
