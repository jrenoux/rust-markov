use std::{collections::{HashMap, HashSet}, hash::Hash};
use super::{reward_model::{MatrixReward, RewardModel}, transition_model::{MatrixTransition, TransitionModel}};
use float_cmp;
pub struct MDPAgent<S, A, T, R> where
    S: Eq + Hash,
    A: Eq + Hash,
    T: TransitionModel<S, A>,
    R: RewardModel<S, A> {
        states: HashSet<S>,
        actions: HashSet<A>,
        transitions: T,
        rewards: R,
        discount: f64,
        transition_values: HashMap<S, HashMap<A, HashMap<S, f64>>>,
        reward_values: HashMap<S, HashMap<A, HashMap<S, f64>>>,
}

impl<S, A, T, R> MDPAgent<S, A, T, R>  where 
    S: Eq + Hash,
    A: Eq + Hash,
    T: TransitionModel<S, A>,
    R: RewardModel<S, A> {
    pub fn new(states: HashSet<S>, actions: HashSet<A>, transitions: T, rewards: R, discount: f64) -> Self {
        MDPAgent {
            states,
            actions,
            transitions,
            rewards,
            discount,
            transition_values: HashMap::new(),
            reward_values: HashMap::new(),
        }

    }

    pub fn new_matrix(nb_states: usize, nb_actions: usize, t: T, r: R, discount: f64) -> MDPAgent<usize, usize, T, R> where 
    T: TransitionModel<usize, usize>,
    R: RewardModel <usize, usize> {
        // create a hashset from enumeration 
        let state_set: HashSet<usize> = (0..nb_states).collect();
        let action_set: HashSet<usize> = (0..nb_actions).collect();
        MDPAgent {
            states: state_set,
            actions: action_set,
            transitions: t,
            rewards: r,
            discount,
            transition_values: HashMap::new(),
            reward_values: HashMap::new(),
        }
    }

    /**
    This function validates the MDP (check that the transition is a probability function). 
    returns: true if the MDP is valid, false otherwise
     */
    pub fn validate(&self) -> bool{ 
        let mut sum_transition;
        for s1 in &self.states{
            for a in &self.actions {
                sum_transition = 0.0; // resets the transition for this state
                for s2 in &self.states {
                    // TODO fill the cache
                    
                    sum_transition = sum_transition + self.transitions.get_transition(s1, a, s2);
                }
                if !float_cmp::approx_eq!(f64, sum_transition, 1.0, ulps = 4) {
                    // if the values don't sum to 1 for this s1 and a
                    return false;
                }
            }
        }
        return true;
    }
}

////////////////////////////////////////////////////////////////////////////
// Unit Tests                                                             //
////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test{
    use std::usize;

    use crate::mdp::{reward_model::MatrixReward, transition_model::MatrixTransition};

    use super::*;

    #[test]
    fn create_valid_matrix_mdp() {
        let transition_array = vec![
                //s1
                vec![vec![0.2, 0.8, 0.0], //a1
                     vec![0.3, 0.3, 0.4]],
                // s2
                vec![vec![0.8, 0.1, 0.1],
                     vec![0.1, 0.1, 0.8]],
                //s3
                vec![vec![0.2, 0.7, 0.1],
                     vec![0.5, 0.0, 0.5]]

            ];
        
        let t: MatrixTransition = MatrixTransition::new(transition_array);

        let reward_array = vec![
                //s1
                vec![vec![0.0, 0.0, 10.0], //a1
                     vec![0.0, 5.0, 0.0]],
                // s2
                vec![vec![0.0, 0.0, 0.0],
                     vec![0.0, 0.0, 0.0]],
                //s3
                vec![vec![0.0, -10.0, 0.0],
                     vec![0.0, -10.0, 0.0]]
            ];

        let r: MatrixReward = MatrixReward::new(reward_array);

        let mdp_agent: MDPAgent<usize, usize, MatrixTransition, MatrixReward> = MDPAgent::new_matrix(3, 2, t, r, 0.9);

        assert!(mdp_agent.validate());
        }

    #[test]
    #[should_panic]
    fn create_invalid_matrix_mdp() {
        let transition_array = vec![
            //s1
            vec![vec![0.2, 0.8, 0.0], //a1
                 vec![0.3, 0.3, 0.4]],
            // s2
            vec![vec![0.8, 0.1, 0.1],
                 vec![0.1, 0.1, 0.8]],
            //s3
            vec![vec![0.2, 0.8, 0.1],
                 vec![0.5, 0.0, 0.5]]

        ];

        let t = MatrixTransition::new(transition_array);

        let reward_array = vec![
                //s1
                vec![vec![0.0, 0.0, 10.0], //a1
                     vec![0.0, 5.0, 0.0]],
                // s2
                vec![vec![0.0, 0.0, 0.0],
                     vec![0.0, 0.0, 0.0]],
                //s3
                vec![vec![0.0, -10.0, 0.0],
                     vec![0.0, -10.0, 0.0]]
            ];

        let r = MatrixReward::new(reward_array);

        let mdp_agent = MDPAgent::new_matrix(3, 2, t, r, 0.9);

        assert!(mdp_agent.validate());

    }

}
