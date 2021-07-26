use std::collections::HashMap;

// use crate::mdp::AbstractMDP;

// pub fn q_value<T>(mdp: &T, s: usize, a: usize, utility_vector: &Vec<f32>) -> f32 
// where T: AbstractMDP {
//     let mut sum = 0.0;
//     for s2 in 0..(*mdp).get_nb_states() {
// 	sum = sum + (*mdp).get_transition_probabilitiy(s, a, s2) * ((*mdp).get_reward(s, a, s2) + (*mdp).get_discount_factor() * (*utility_vector)[s2]);
//     }

//     return sum;   
// }


// ////////////////////////////////////////////////////////////////////////////
// // Tests                                                                  //
// ////////////////////////////////////////////////////////////////////////////
// #[cfg(test)]
// mod test {
//     use super::*;
    
//     use crate::mdp::simple_mdp::SimpleMDP;

//     fn create_mdp() -> SimpleMDP{
// 	let states = 2;
// 	let actions = 3;
// 	let transitions = vec![
// 	    vec![vec![0.2, 0.8], vec![0.4, 0.6], vec![0., 1.]],  //s1 = 1
// 	    vec![vec![0.8, 0.2], vec![0.3, 0.7], vec![1., 0.]]   //s1 = 2
// 	];
// 	let rewards = vec![
// 	    vec![vec![1., 1.], vec![1., 1.], vec![1., 1.]],
// 	    vec![vec![0., 0.], vec![0., 0.], vec![0., 0.]]
// 	];

// 	SimpleMDP::new(states, actions, transitions, rewards, 0.95)
//     }

//     #[test]
//     fn q_value_from_zero_vector() {
// 	let mdp = create_mdp();

// 	let init_utility_vector = vec![0.0, 0.0];

// 	let value = q_value(&mdp, 0, 0, &init_utility_vector);
// 	assert!((value - 1.0).abs() < 0.001);

// 	// TODO continue with other state / action combinations
	
//     }

//     #[test]
//     fn q_value_from_non_zero_vector() {
// 	let mdp = create_mdp();

// 	let utility_vector = vec![5.0, 3.0];

// 	let value = q_value(&mdp, 0, 0, &utility_vector);
// 	assert!((value - 4.23).abs() < 0.001);

// 	// TODO continue with othe state / action combinations
	
	
//     }

// }

pub fn array_to_hashmap<T, R, F>(array: &[T], f: F) -> HashMap<usize, R> 
where F: Fn(&T) -> R {
    array.iter()
            .enumerate()
            .map(|(i, x)|
            {
                (i, f(x))
            })
            .collect()
}

pub fn vector_to_hashmap<T, R, F>(vector: &Vec<T>, f: F) -> HashMap<usize, R>
where F: Fn(&T) -> R {
    vector.iter()
            .enumerate()
            .map(|(i, x)|
            {
                (i, f(x))
            })
            .collect()
}