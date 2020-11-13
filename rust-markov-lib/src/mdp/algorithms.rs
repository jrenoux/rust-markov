

////////////////////////////////////////////////////////////////////////////
// Value Iteration                                                        //
////////////////////////////////////////////////////////////////////////////

pub mod ValueIteration {
    use crate::mdp::MDP;
    use crate::utils::*;
    
    pub fn solve<T>(mdp:&T, epsilon:f32)
    where T: MDP{
	// Vector of utilities
	let utility_vector:Vec<f32> = vec![0.0; mdp.get_nb_states()];
	let mut previous_utility_vector:Vec<f32>;

	// Policy. Contains the indices of the actions
	let mut policy:Vec<u32> = vec![0; mdp.get_nb_states()];

	// maximum relative change in utility of any state
	let mut delta = 0.0;


	loop {
	    previous_utility_vector = utility_vector.clone();
	    
	    for s in 0..(*mdp).get_nb_states() {
		// update the q-value function
		let mut max_utility = 0;
		// for each possible action
		for a in 0..mdp.get_nb_actions() {
		    let utility = q_value(mdp, s, a, &utility_vector);
		}

		// Check the utility change
		if (utility_vector[s] - previous_utility_vector[s]).abs() > delta {
		    delta = (utility_vector[s] - previous_utility_vector[s]).abs();
		}
	
	    }

	    // stopping condition
	    if delta <= (epsilon * (1.0 - mdp.get_discount_factor()) / mdp.get_discount_factor()) { break; }
	}
	
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
    fn hello_test() {
	
	ValueIteration::solve(&create_mdp(), 0.01);
    }
	
}
