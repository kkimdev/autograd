/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// TODO Setup contributor license agreement.

//! this is context class to use autograd.

extern crate std;

use super::float::FloatCratePrivate;

// TODO #[inline] where appropriate.

pub trait Context<T>: std::marker::Sized where T: std::num::Float {
    // public functions

    fn new_variable(&self, value: T) -> super::float::Float<T, Self> {
        FloatCratePrivate::new(value, <Self as Context<T>>::get_new_variable_index())
    }

    fn differentiate(&self, float: super::float::Float<T, Self>) {
        // TODO check if it exceeded the capacity.
        // TODO The current implementation is not performant and dirty.
        unsafe {
            // let count = <Self as Context<T>>::get_recorded_variables_count();
            for i in (0..(*<Self as Context<T>>::get_recorded_variables_count())) {
                *<Self as Context<T>>::get_result_derivatives().offset(i as isize) = std::num::Float::zero();
            }

            *<Self as Context<T>>::get_result_derivatives().offset(float.float_get_index() as isize) = std::num::Float::one();
            for i in (0..(*<Self as Context<T>>::get_recorded_entries_count())).rev() {
                let lhs_index = *<Self as Context<T>>::get_lhs_indices().offset(i as isize);
                let rhs_index = *<Self as Context<T>>::get_rhs_indices().offset(i as isize);
                *<Self as Context<T>>::get_result_derivatives().offset(rhs_index as isize) =
                    *<Self as Context<T>>::get_result_derivatives().offset(rhs_index as isize)
                    + (*<Self as Context<T>>::get_result_derivatives().offset(lhs_index as isize)
                       * *<Self as Context<T>>::get_adjoints().offset(i as isize));

                let t = *<Self as Context<T>>::get_result_derivatives().offset(i as isize);
            }
        }
    }

    fn get_derivative(&self, float: super::float::Float<T, Self>) -> T {
        let float_index_offset = float.float_get_index() as isize;
        unsafe {
            *<Self as Context<T>>::get_result_derivatives().offset(float_index_offset)
        }
    }

    // Crate private functions
    // TODO Move private functions to a seperate trait.

    fn get_new_variable_index() -> usize {
        let count = <Self as Context<T>>::get_recorded_variables_count();
        let index = *count;
        *count += 1;
        index
    }

    fn get_new_entry_index() -> usize {
        let count = <Self as Context<T>>::get_recorded_entries_count();
        let index = *count;
        *count += 1;
        index
    }

    fn unary_operation(adjoint: T, rhs_index: usize) -> usize {
        let lhs_index = <Self as Context<T>>::get_new_variable_index();
        let recorded_entries_count_offset = <Self as Context<T>>::get_new_entry_index() as isize;
        unsafe {
            *<Self as Context<T>>::get_adjoints().offset(recorded_entries_count_offset) = adjoint;
            *<Self as Context<T>>::get_lhs_indices().offset(recorded_entries_count_offset) = lhs_index;
            *<Self as Context<T>>::get_rhs_indices().offset(recorded_entries_count_offset) = rhs_index;
        }
        lhs_index
    }

    fn binary_operation(adjoints: &[T; 2],
                        rhs_indices: &[usize; 2]) -> usize {
        let lhs_index = <Self as Context<T>>::get_new_variable_index();
        let recorded_entries_count_offset_1 = <Self as Context<T>>::get_new_entry_index() as isize;
        let recorded_entries_count_offset_2 = <Self as Context<T>>::get_new_entry_index() as isize;

        unsafe {
            // TODO is indexing inefficient?
            *<Self as Context<T>>::get_adjoints().offset(recorded_entries_count_offset_1) = adjoints[0];
            *<Self as Context<T>>::get_lhs_indices().offset(recorded_entries_count_offset_1) = lhs_index;
            *<Self as Context<T>>::get_rhs_indices().offset(recorded_entries_count_offset_1) = rhs_indices[0];

            *<Self as Context<T>>::get_adjoints().offset(recorded_entries_count_offset_2) = adjoints[1];
            *<Self as Context<T>>::get_lhs_indices().offset(recorded_entries_count_offset_2) = lhs_index;
            *<Self as Context<T>>::get_rhs_indices().offset(recorded_entries_count_offset_2) = rhs_indices[1];
        }
        lhs_index
    }

    fn get_recorded_variables_count() -> &'static mut usize;
    fn get_recorded_entries_count() -> &'static mut usize;
    // TODO use 'static lifetime instead?
    fn get_adjoints<'a>() -> &'a mut*mut T;
    fn get_lhs_indices<'a>() -> &'a mut*mut usize;
    fn get_rhs_indices<'a>() -> &'a mut*mut usize;
    fn get_result_derivatives<'a>() -> &'a mut*mut T;
}

#[macro_export]
macro_rules! new_autograd_context {
    ($T:ty, $capacity:expr) => (
        {
            struct ContextImpl {
                capacity: usize,
                mutex_guard: std::sync::MutexGuard<'static ()>,
            }

            impl $crate::Context<$T> for ContextImpl {
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
                fn get_adjoints<'a>() -> &'a mut*mut $T {
                    #[thread_local]
                    static mut ptr : *mut $T = 0 as *mut $T;
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

                fn get_result_derivatives<'a>() -> &'a mut*mut $T {
                    #[thread_local]
                    static mut ptr : *mut $T = 0 as *mut $T;
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
                    let T_size = capacity * std::mem::size_of::<$T>();
                    // std::rt::heap::allocate(T_size, mem::min_align_of::<T>())

                    unsafe {
                        *<Self as $crate::Context<$T>>::get_recorded_variables_count() = 0;
                        *<Self as $crate::Context<$T>>::get_recorded_entries_count() = 0;
                        *<Self as $crate::Context<$T>>::get_adjoints() = std::rt::heap::allocate(T_size, std::mem::align_of::<$T>()) as *mut $T;
                        *<Self as $crate::Context<$T>>::get_lhs_indices() = std::rt::heap::allocate(usize_size, std::mem::align_of::<usize>()) as *mut usize;
                        *<Self as $crate::Context<$T>>::get_rhs_indices() = std::rt::heap::allocate(usize_size, std::mem::align_of::<usize>()) as *mut usize;
                        // TODO we don't have to allocate get_result_derivatives now, isn't it?
                        *<Self as $crate::Context<$T>>::get_result_derivatives() = std::rt::heap::allocate(T_size, std::mem::align_of::<$T>()) as *mut $T;
                    }
                    context
                }
            }

            impl std::ops::Drop for ContextImpl {
                fn drop(&mut self) {
                    // *Context::<$T>::get_recorded_variables_count(None::<Self>) = 0;
                    // *Context::<$T>::get_adjoints(None::<Self>) = 0;
                    // TODO implement.

                    let usize_size = self.capacity * std::mem::size_of::<usize>();
                    let T_size = self.capacity * std::mem::size_of::<$T>();

                    unsafe {
                        std::rt::heap::deallocate(*<Self as $crate::Context<$T>>::get_adjoints() as *mut u8, T_size, std::mem::align_of::<$T>());
                        std::rt::heap::deallocate(*<Self as $crate::Context<$T>>::get_lhs_indices() as *mut u8, usize_size, std::mem::align_of::<usize>());
                        std::rt::heap::deallocate(*<Self as $crate::Context<$T>>::get_rhs_indices() as *mut u8, usize_size, std::mem::align_of::<usize>());
                        std::rt::heap::deallocate(*<Self as $crate::Context<$T>>::get_result_derivatives() as *mut u8, T_size, std::mem::align_of::<$T>());
                    }
                }
            }

            ContextImpl::new($capacity)
        }
    )
}
