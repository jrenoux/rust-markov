use std::{collections::HashMap, hash::Hash};
use crate::utils::*;
pub trait TransitionModel<S, A> where 
    S: Eq + Hash, 
    A: Eq + Hash {
    fn get_transition(&self, s1: &S, a: &A, s2: &S) -> f64;
    fn get_all_transitions(&self, s1: &S, a: &A) -> &HashMap<S, f64>;
}

pub struct MatrixTransition {
    t: HashMap<usize, HashMap<usize, HashMap <usize, f64>>>
}



impl MatrixTransition {
    pub fn new(transition_array: Vec<Vec<Vec<f64>>>) -> Self {
        let transition = 
                vector_to_hashmap(&transition_array,
                    |x| vector_to_hashmap( x, 
                                    |y| vector_to_hashmap(y, |z| *z)));
        MatrixTransition {
            t: transition
        }
    }
}


impl TransitionModel<usize, usize> for MatrixTransition {
    fn get_transition(&self, s1: &usize, a: &usize, s2: &usize) -> f64{   
        // panics if the combination s1 a s2 is not in the hashmap
        *self.t.get(s1).unwrap().get(a).unwrap().get(s2).unwrap()
    }

    fn get_all_transitions(&self, s1: &usize, a: &usize) -> &HashMap<usize, f64>{
        self.t.get(s1).unwrap().get(a).unwrap()    
    }
}