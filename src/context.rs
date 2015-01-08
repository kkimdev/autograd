//! this is context class to use autograd.

extern crate std;

// TODO #[inline] where appropriate.

pub trait Context<T: std::num::Float>: std::marker::Sized {
    // public functions

    fn new() -> Self {
        // TODO implement correctly.
        // let size: usize = (std::u32::MAX as usize) / 2;
        Context::<T>::new_tmp()
    }

    fn new_variable(&self, value: T) -> super::float::Float<T, Self> {
        super::float::float_new(value, <Self as Context<T>>::get_new_variable_index())
    }

    fn differentiate(&self, float: super::float::Float<T, Self>) {
        unsafe {
            // let count = <Self as Context<T>>::get_recorded_variables_count();
            *<Self as Context<T>>::get_result_derivatives().offset(super::float::float_get_index(&float) as isize) = std::num::Float::one();
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
        let float_index_offset = super::float::float_get_index(&float) as isize;
        unsafe {
            *<Self as Context<T>>::get_result_derivatives().offset(float_index_offset)
        }
    }

    // Private functions

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

    // TODO the Option arguments and above std::marker::Sized are
    // a hack due to the Rust compiler's limitation.
    // This should be possible ideally. http://is.gd/QelquA
    // TODO just return raw memory instead of vector?
    fn get_recorded_variables_count<'a>() -> &'a mut usize;
    fn get_recorded_entries_count<'a>() -> &'a mut usize;
    fn get_adjoints<'a>() -> &'a mut*mut T;
    fn get_lhs_indices<'a>() -> &'a mut*mut usize;
    fn get_rhs_indices<'a>() -> &'a mut*mut usize;
    fn get_result_derivatives<'a>() -> &'a mut*mut T;

    fn new_tmp() -> Self;
}

pub struct ContextImpl {
    // TODO this is to keep the struct construction private. Remove when we don't need it.
    _private: (),
}

impl Context<f32> for ContextImpl {
    fn get_recorded_variables_count<'a>() -> &'a mut usize {
        #[thread_local]
        static mut ptr : usize = 0;
        unsafe {
            &mut ptr
        }
    }
    fn get_recorded_entries_count<'a>() -> &'a mut usize {
        #[thread_local]
        static mut ptr : usize = 0;
        unsafe {
            &mut ptr
        }
    }
    fn get_adjoints<'a>() -> &'a mut*mut f32 {
        #[thread_local]
        static mut ptr : *mut f32 = 0 as *mut f32;
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

    fn get_result_derivatives<'a>() -> &'a mut*mut f32 {
        #[thread_local]
        static mut ptr : *mut f32 = 0 as *mut f32;
        unsafe {
            &mut ptr
        }
    }

    fn new_tmp() -> Self {
        // TODO implement

        let capacity: usize = 1000;

        // TODO use checked_mul?
        //      example : let usize_size = capacity.checked_mul(std::mem::size_of::<usize>()).expect("capacity overflow");
        let usize_size = capacity * std::mem::size_of::<usize>();
        let f32_size = capacity * std::mem::size_of::<f32>();
        // std::rt::heap::allocate(f32_size, mem::min_align_of::<T>())

        unsafe {
            *<Self as Context<f32>>::get_recorded_variables_count() = 0;
            *<Self as Context<f32>>::get_recorded_entries_count() = 0;
            *<Self as Context<f32>>::get_adjoints() = std::rt::heap::allocate(f32_size, std::mem::align_of::<f32>()) as *mut f32;
            *<Self as Context<f32>>::get_lhs_indices() = std::rt::heap::allocate(usize_size, std::mem::align_of::<usize>()) as *mut usize;
            *<Self as Context<f32>>::get_rhs_indices() = std::rt::heap::allocate(usize_size, std::mem::align_of::<usize>()) as *mut usize;
            // TODO we don't have to allocate get_result_derivatives now, isn't it?
            *<Self as Context<f32>>::get_result_derivatives() = std::rt::heap::allocate(f32_size, std::mem::align_of::<f32>()) as *mut f32;
        }
        ContextImpl{_private: ()}
    }
}

// TODO implement safe lock mutex.
// impl ContextImpl {
//     fn get_mutex() {
//         #[thread_local]
//         static MUTEX : std::sync::StaticMutex = std::sync::MUTEX_INIT;
//     }
// }

impl std::ops::Drop for ContextImpl {
    fn drop(&mut self) {
        // *Context::<f32>::get_recorded_variables_count(None::<Self>) = 0;
        // *Context::<f32>::get_adjoints(None::<Self>) = 0;
        // TODO implement.

        let capacity: usize = 1000;
        let usize_size = capacity * std::mem::size_of::<usize>();
        let f32_size = capacity * std::mem::size_of::<f32>();

        unsafe {
            std::rt::heap::deallocate(*<Self as Context<f32>>::get_adjoints() as *mut u8, f32_size, std::mem::align_of::<f32>());
            std::rt::heap::deallocate(*<Self as Context<f32>>::get_lhs_indices() as *mut u8, usize_size, std::mem::align_of::<usize>());
            std::rt::heap::deallocate(*<Self as Context<f32>>::get_rhs_indices() as *mut u8, usize_size, std::mem::align_of::<usize>());
            std::rt::heap::deallocate(*<Self as Context<f32>>::get_result_derivatives() as *mut u8, f32_size, std::mem::align_of::<f32>());
        }
    }
}
