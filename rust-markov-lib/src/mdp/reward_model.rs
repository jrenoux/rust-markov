use core::hash::Hash;
use std::{collections::HashMap};

use crate::utils::vector_to_hashmap;

pub trait RewardModel<S, A>  where 
    S: Eq + Hash, 
    A: Eq + Hash {
    fn get_reward(&self, s1: &S, a: &A, s2: &S) -> f64; 
}

pub struct MatrixReward{
    r: HashMap<usize, HashMap<usize, HashMap <usize, f64>>>
}

impl MatrixReward{
    pub fn new(reward_array: Vec<Vec<Vec<f64>>>) -> Self {
        let reward = vector_to_hashmap(&reward_array,|x| 
                                                      vector_to_hashmap( x, |y| 
                                                           vector_to_hashmap(y, |z| *z)));
        MatrixReward {
            r: reward
        }
            
    }
}

impl RewardModel<usize, usize> for MatrixReward {
    fn get_reward(&self, s1: &usize, a: &usize, s2: &usize) -> f64 {
        *(self.r.get(s1).unwrap().get(a).unwrap().get(s2).unwrap())       
    }
}
