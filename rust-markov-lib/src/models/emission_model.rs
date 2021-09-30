use std::hash::Hash;
use std::{collections::{HashMap}};
use crate::utils::*;
pub trait EmissionModel<S, A, O> where
S: Eq + Hash,
A: Eq + Hash,
O: Eq + Hash {
    fn get_emission(&self, s: &S, a: &A, o: &O) -> f64;

}

pub struct MatrixEmission {
    e: HashMap<usize, HashMap<usize, HashMap <usize, f64>>>
}

impl MatrixEmission {
    pub fn new(emission_array: Vec<Vec<Vec<f64>>>) -> Self {
        let emissions = vector_to_hashmap(&emission_array,
                            |x| vector_to_hashmap( x, 
                                |y| vector_to_hashmap(y, |z| *z)));
        MatrixEmission {
            e: emissions
        }
    }
}

impl EmissionModel<usize, usize, usize> for MatrixEmission {
    fn get_emission(&self, s: &usize, a: &usize, o: &usize) -> f64 {
        *self.e.get(s).unwrap().get(a).unwrap().get(o).unwrap()
    }
}