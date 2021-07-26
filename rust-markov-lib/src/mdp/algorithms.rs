// use crate::mdp::MDP;
// use crate::utils::*;

// use super::AbstractMDP;

// ////////////////////////////////////////////////////////////////////////////
// // Value Iteration                                                        //
// ////////////////////////////////////////////////////////////////////////////

// pub struct ValueIteration<'a, T>
// where T:AbstractMDP{
//     // Parameters
//     epsilon:f32,
//     mdp: &'a T,
    
//     // Internal
//     value_vector:std::option::Option<Vec<f32>>,
//     policy:std::option::Option<Vec<usize>>,
    
// }

// impl<'a, T> ValueIteration<'a, T>
// where T:AbstractMDP {

//     pub fn new(mdp: &'a T, eps:f32) -> ValueIteration<'a, T>{
// 	ValueIteration {
// 	    epsilon: eps,
// 	    mdp: mdp,
// 	    value_vector: None,
// 	    policy: None,
// 	}
//     }
 
//     pub fn solve(&mut self) {
	
// 	// initialize the value_vector
// 	let mut utility_vector = vec![0.0; self.mdp.get_nb_states()];
// 	//  and the policy_vector
// 	let mut policy_vector = vec![0; self.mdp.get_nb_states()];
	
// 	// Vector of utilities
// 	let mut previous_utility_vector:Vec<f32>;

// 	// maximum relative change in utility of any state
// 	let mut delta;
 

// 	loop {
// 	    delta = 0.0;
// 	    previous_utility_vector = utility_vector.clone();
	    
// 	    for s in 0..self.mdp.get_nb_states() {
// 		// update the q-value function
// 		let mut max_utility = None;
// 		let mut best_action = None;
// 		// for each possible action
// 		for a in 0..self.mdp.get_nb_actions() {
// 		    let utility = q_value(self.mdp, s, a, &previous_utility_vector);
		    
// 		    match max_utility {
// 			Some(mu) => {
// 			    if utility > mu {
// 				max_utility = Some(utility);
// 				best_action = Some(a);
// 			    }
// 			},
// 			None => {
// 			    max_utility = Some(utility);
// 			    best_action = Some(a);
// 			},
// 		    };
// 		}

// 		utility_vector[s] = max_utility.unwrap();
// 		policy_vector[s] = best_action.unwrap();

// 		// Check the utility change
// 		let new_delta = (utility_vector[s] - previous_utility_vector[s]).abs();
// 		if new_delta > delta {
// 		    delta = (utility_vector[s] - previous_utility_vector[s]).abs();
// 		}
	
// 	    }
// 	    // stopping condition
// 	    if delta <= (self.epsilon * (1.0 - self.mdp.get_discount_factor()) / self.mdp.get_discount_factor()) { break; }
// 	}
// 	self.value_vector = Some(utility_vector);
// 	self.policy = Some(policy_vector);
//     }

//     pub fn get_value_vector(&self) -> &std::option::Option<Vec<f32>> {
// 	return &self.value_vector;
//     }

//     pub fn get_policy(&self) -> &std::option::Option<Vec<usize>> {
// 	return &self.policy;
//     }

    
// }


// ////////////////////////////////////////////////////////////////////////////
// // Policy Iteration                                                        //
// ////////////////////////////////////////////////////////////////////////////
// pub struct PolicyIteration<'a, T>
// where T:AbstractMDP {
//     // Parameters
//     epsilon:f32,
//     mdp: &'a T,
    
//     // Internal
//     value_vector:std::option::Option<Vec<f32>>,
//     policy_vector:std::option::Option<Vec<usize>>,
// }

// impl<'a, T> PolicyIteration<'a, T>
// where T: AbstractMDP {
//     pub fn new(mdp: &'a T, eps:f32) -> PolicyIteration<'a, T>{
// 	PolicyIteration {
// 	    epsilon: eps,
// 	    mdp: mdp,
// 	    value_vector: None,
// 	    policy_vector: None,
// 	}
//     }

//     fn policy_evaluation(&self, v_vector:Vec<f32>) -> Vec<f32>{
// 	// TODOOOOO

	
// 	v_vector
//     }

//     pub fn solve(&mut self) {
// 	// initialize the value_vector
// 	let mut v_vector = vec![0.0; self.mdp.get_nb_states()];
// 	//  and the policy_vector
// 	let mut p_vector = vec![0; self.mdp.get_nb_states()];

// 	let mut unchanged:bool;

// 	loop {
// 	    // Policy evaluation step
// 	    v_vector = self.policy_evaluation(v_vector);
// 	    unchanged = true;
	    
// 	    // Policy improvement step
// 	    for s in 0..self.mdp.get_nb_states() {
// 		let mut max_utility = None;
// 		let mut best_action = None;
// 		for a in 0..self.mdp.get_nb_actions() {
// 		    let utility = q_value(self.mdp, s, a, &v_vector);

// 		    match max_utility {
// 			Some(mu) => {
// 			    if utility > mu {
// 				max_utility = Some(utility);
// 				best_action = Some(a);
// 			    }
// 			}
// 			None => {
// 			    max_utility = Some(utility);
// 			    best_action = Some(a);
// 			}
// 		    }
// 		}

// 		// Ensure that best_cation is not None, and panic if it is
// 		if best_action == None {
// 		    panic!("Internal Error. No action found in PolicyIteration.solve(). If this message appears, please fill in an issue report.");
// 		}
		
// 		if q_value(self.mdp, s, best_action.unwrap(), &v_vector) > q_value(self.mdp, s, p_vector[s], &v_vector) {
// 		    p_vector[s] = best_action.unwrap();
// 		    unchanged = false;
// 		}
		
// 	    }
	       

// 	    // break condition
// 	     if unchanged { break; }
// 	}
	
// 	self.value_vector = Some(v_vector);
// 	self.policy_vector = Some(p_vector);
	
	
//     }

//     pub fn get_value_vector(&self) -> &std::option::Option<Vec<f32>> {
// 	return &self.value_vector;
//     }

//     pub fn get_policy(&self) -> &std::option::Option<Vec<usize>> {
// 	return &self.policy_vector;
//     }

// }



// ////////////////////////////////////////////////////////////////////////////
// // Tests                                                                  //
// ////////////////////////////////////////////////////////////////////////////
// #[cfg(test)]
// mod test{
//     use float_cmp::*;
//     use super::*;
//     use crate::mdp::simple_mdp::SimpleMDP;

//     fn create_student_mdp() -> SimpleMDP {
// 	let states = vec!["facebook".to_string(), "class1".to_string(), "pub".to_string(), "pass".to_string()];
// 	let actions = vec!["browse".to_string(), "study".to_string(), "go_pub".to_string()];
// 	let nb_actions = actions.len();
// 	let nb_states = states.len();

// 	let mut transitions = vec![
// 	    vec![vec![0.0; nb_states] ; nb_actions] ; nb_states
// 	];

// 	let mut reward = vec![
// 	    vec![vec![0.0; nb_states] ; nb_actions] ; nb_states
// 	];

// 	// init the transitions -> To correct
// 	// facebook starting state
// 	// browse -> facebook
// 	transitions[0][0][0] = 1.0;
// 	// study -> class1 (0.7) / facebook (0.3)
// 	transitions[0][1][1] = 0.7;
// 	transitions[0][1][0] = 0.3;
// 	// go_pub -> pub
// 	transitions[0][2][2] = 1.0;
	
// 	// class1 starting state
// 	// browse -> facebook
// 	transitions[1][0][0] = 1.0;
// 	// study -> pass (0.7) / class1 (0.3)
// 	transitions[1][1][3] = 0.7;
// 	transitions[1][1][1] = 0.3;	    
// 	// go_pub -> pub
// 	transitions[1][2][2] = 1.0;

// 	// pub starting state
// 	// browse -> facebook (0.5) / pub (0.5)
// 	transitions[2][0][0] = 0.2;
// 	transitions[2][0][2] = 0.8;
// 	// study -> class1 (0.2) / pub (0.8)
// 	transitions[2][1][1] = 0.4;
// 	transitions[2][1][2] = 0.6;
// 	// go_pub -> pub
// 	transitions[2][2][2] = 1.0;


// 	// pass starting state
// 	// browse -> pass
// 	transitions[3][0][3] = 1.0;
// 	// study -> pass
// 	transitions[3][1][3] = 1.0;
// 	// go_pub -> pass
// 	transitions[3][2][3] = 1.0;

// 	// facebook
// 	reward[0][1][1] = -1.0;
	
// 	// class1
// 	reward[1][0][0] = 1.0;
// 	reward[1][1][3] = 10.0;
// 	reward[1][2][2] = 2.0;

// 	// pub
// 	reward[2][1][1] = -1.0;
	

// 	// eprintln!("transitions: ");
// 	// eprintln!("{:?}", transitions);

// 	// eprintln!("rewards: ");
// 	// eprintln!("{:?}", reward);
	
// 	SimpleMDP::new_named(states, actions, transitions, reward, 0.95)	
	
//     }

//     #[test]
//     fn value_iteration_solve() {
// 	let mdp = create_student_mdp();
// 	let mut solver = ValueIteration::new(&mdp, 0.01);
// 	solver.solve();
// 	let optimal_utility_vector = solver.get_value_vector();
// 	let optimal_policy = solver.get_policy();

// 	let expected_utility_vector = vec![8.126558, 9.79021, 7.72112, 0.0];
// 	let expected_policy = vec![1, 1, 1, 0];

// 	match optimal_utility_vector {
// 	    None => panic!("No optimal policy has been found"),
// 	    Some(v) => {
// 		for index in 0..v.len() {
// 		    assert!(approx_eq!(f32, v[index], expected_utility_vector[index], ulps=4))
// 		}
// 	    }
// 	}
	
// 	match optimal_policy {
// 	    None => panic!("No optimal policy has been found"),
// 	    Some(p) => {
// 		for index in 0..p.len() {
// 		    assert!(p[index] == expected_policy[index])
// 		}
// 	    }
// 	}
//     }
	
// }
