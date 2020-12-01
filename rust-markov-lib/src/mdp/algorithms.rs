use crate::mdp::MDP;
use crate::utils::*;

////////////////////////////////////////////////////////////////////////////
// Value Iteration                                                        //
////////////////////////////////////////////////////////////////////////////

pub struct ValueIteration<'a, T>
where T:MDP{
    // Parameters
    epsilon:f32,
    mdp: &'a T,
    
    // Internal
    value_vector:std::option::Option<Vec<f32>>,
    policy:std::option::Option<Vec<usize>>,

    // To make it private
    _private:()
}

impl<'a, T> ValueIteration<'a, T>
where T:MDP {

    pub fn new(mdp: &'a T, eps:f32) -> ValueIteration<'a, T>{
	ValueIteration {
	    epsilon: eps,
	    mdp: mdp,
	    value_vector: None,
	    policy: None,
	    _private: ()
	}
    }
 
    pub fn solve(&mut self)
    where T: MDP{
	
	// initialize the value_vector
	let mut utility_vector = vec![0.0; self.mdp.get_nb_states()];
	//  and the policy_vector
	let mut policy_vector = vec![0; self.mdp.get_nb_states()];
	
	// Vector of utilities
	let mut previous_utility_vector:Vec<f32>;

	// maximum relative change in utility of any state
	let mut delta = 0.0;
 

	loop {
	    delta = 0.0;
	    previous_utility_vector = utility_vector.clone();
	    
	    for s in 0..self.mdp.get_nb_states() {
		// update the q-value function
		let mut max_utility = None;
		let mut best_action = None;
		// for each possible action
		for a in 0..self.mdp.get_nb_actions() {
		    let utility = q_value(self.mdp, s, a, &previous_utility_vector);
		    match max_utility {
			None => {
			    max_utility = Some(utility);
			    best_action = Some(a);
			},
			Some(mu) => {
			    if utility > mu {
				max_utility = Some(utility);
				best_action = Some(a);
			    }
			},
		    };
		}

		utility_vector[s] = max_utility.unwrap();
		policy_vector[s] = best_action.unwrap();

		// Check the utility change
		let new_delta = (utility_vector[s] - previous_utility_vector[s]).abs();
		if new_delta > delta {
		    delta = (utility_vector[s] - previous_utility_vector[s]).abs();
		}
	
	    }
	    // stopping condition
	    if delta <= (self.epsilon * (1.0 - self.mdp.get_discount_factor()) / self.mdp.get_discount_factor()) { break; }
	}
	self.value_vector = Some(utility_vector);
	self.policy = Some(policy_vector);
    }

    pub fn get_value_vector(&self) -> &std::option::Option<Vec<f32>> {
	return &self.value_vector;
    }

    pub fn get_policy(&self) -> &std::option::Option<Vec<usize>> {
	return &self.policy;
    }

    
}




////////////////////////////////////////////////////////////////////////////
// Policy Iteration                                                        //
////////////////////////////////////////////////////////////////////////////


pub mod PolicyIteration {
    
}


////////////////////////////////////////////////////////////////////////////
// Tests                                                                  //
////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test{
    use super::*;
    use crate::mdp::simple_mdp::SimpleMDP;

    fn create_mdp() -> SimpleMDP{
	let states = 2;
	let actions = 3;
	let transitions = vec![
	    vec![vec![0.2, 0.8], vec![0.4, 0.6], vec![0., 1.]],  //s1 = 1
	    vec![vec![0.8, 0.2], vec![0.3, 0.7], vec![1., 0.]]   //s1 = 2
	];
	let rewards = vec![
	    vec![vec![1., 1.], vec![1., 1.], vec![1., 1.]],
	    vec![vec![0., 0.], vec![0., 0.], vec![0., 0.]]
	];

	SimpleMDP::new(states, actions, transitions, rewards, 0.95)
    }

    #[test]
    fn value_iteration_solve() {
	let mdp = create_mdp();
	let mut solver = ValueIteration::new(&mdp, 0.01);
	solver.solve();
	let optimal_utility_vector = solver.get_value_vector();
	let optimal_policy = solver.get_policy();
	eprintln!("Utility_vector:{:?}", optimal_utility_vector);
	eprintln!("optimal_policy:{:?}", optimal_policy);
    }
	
}
