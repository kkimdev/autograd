//! this is context class to use autograd.

extern crate std;



pub struct Context<T> {
    tape: std::cell::RefCell<Tape<T>>,
}

impl <'a, T: std::num::Float> Context<T> {
    pub fn new() -> Context<T> {
        let size: usize = (std::u32::MAX as usize) / 2;
        Context{tape: std::cell::RefCell::new(Tape::<T>::new(size)),}
    }

    pub fn new_variable(&self, value: T) -> super::float::Float<T> {
        super::float::float_new(value, self, get_new_variable_index(self))
    }

    // TODO &float ?
    pub fn differentiate(&self, float: super::float::Float<T>) {
        self.tape.borrow_mut().differentiate(super::float::float_get_index(&float));
    }

    // TODO &float ?
    pub fn get_derivative(&self, float: super::float::Float<T>) -> T {
        self.tape.borrow_mut().get_derivative(super::float::float_get_index(&float))
    }
}

pub fn get_derivative<T: std::num::Float>(context: &Context<T>, index: usize)
 -> T {
    context.tape.borrow_mut().get_derivative(index)
}

pub fn get_new_variable_index<T: std::num::Float>(context: &Context<T>)
 -> usize {
    context.tape.borrow_mut().get_new_variable_index()
}

pub fn unary_operation<T: std::num::Float>(context: &Context<T>, adjoint: T,
                                           rhs_index: usize) -> usize {
    let lhs_index =
        context.tape.borrow_mut().record_unary_operation(adjoint, rhs_index);
    lhs_index
}

pub fn binary_operation<T: std::num::Float>(context: &Context<T>,
                                            adjoints: &[T; 2],
                                            rhs_indices: &[usize; 2]) -> usize {
    let lhs_index =
        context.tape.borrow_mut().record_binary_operation(adjoints,
                                                          rhs_indices);
    lhs_index
}


struct Tape<T> {
    recorded_variables_count: usize,
    adjoints: Vec<T>,
    lhs_indices: Vec<usize>,
    rhs_indices: Vec<usize>,
    result_derivatives: Option<Vec<T>>,
}

impl <T: std::num::Float> Tape<T> {
    fn new(size: usize) -> Tape<T> {
        Tape{recorded_variables_count: 0,
             adjoints: Vec::with_capacity(size),
             lhs_indices: Vec::with_capacity(size),
             rhs_indices: Vec::with_capacity(size),
             result_derivatives: None,}
    }

    fn record_unary_operation(&mut self, adjoint: T, rhs_index: usize)
     -> usize {
        let lhs_index = self.get_new_variable_index();
        self.adjoints.push(adjoint);
        self.lhs_indices.push(lhs_index);
        self.rhs_indices.push(rhs_index);
        lhs_index
    }

    fn record_binary_operation(&mut self, adjoints: &[T; 2],
                               rhs_indices: &[usize; 2]) -> usize {
        let lhs_index = self.get_new_variable_index();
        self.adjoints.push_all(adjoints);
        self.lhs_indices.push_all(&[lhs_index, lhs_index]);
        self.rhs_indices.push_all(rhs_indices);
        lhs_index
    }

    fn get_new_variable_index(&mut self) -> usize {
        let index = self.recorded_variables_count;
        self.recorded_variables_count += 1;
        index
    }

    // TODO This function was written quickly to just work. improve it.
    fn differentiate(&mut self, index: usize) {

        let mut diffs : Vec<T> =
            std::iter::repeat(std::num::Float::zero()).take(self.recorded_variables_count).collect();
        let adjoints_iter = self.adjoints.iter().rev();
        let lhs_indices_iter = self.lhs_indices.iter().rev();
        let rhs_indices_iter = self.rhs_indices.iter().rev();

        diffs[index] = std::num::Float::one();
        for ((&adjoint, &lhs_index), &rhs_index) in
            adjoints_iter.zip(lhs_indices_iter).zip(rhs_indices_iter) {
            diffs[rhs_index] = diffs[rhs_index] + diffs[lhs_index] * adjoint;
        }
        self.result_derivatives = Some(diffs);
    }

    fn get_derivative(&self, index: usize) -> T {
        match self.result_derivatives {
            None => panic!(),
            Some(ref t) => t[index],
        }
    }
}
