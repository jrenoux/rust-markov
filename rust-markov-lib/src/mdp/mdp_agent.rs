use std::{collections::{HashSet}, hash::Hash};
use super::{reward_model::{MatrixReward, RewardModel}, transition_model::{MatrixTransition, TransitionModel}};
use float_cmp;
pub struct MDPAgent<S, A, T, R> where
    S: Eq + Hash,
    A: Eq + Hash,
    T: TransitionModel<S, A>,
    R: RewardModel<S, A> {
        pub states: HashSet<S>,
        pub actions: HashSet<A>,
        pub transitions: T,
        pub rewards: R,
        pub discount: f64
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
        }

    }

    /**
    This function validates the MDP (check that the transition is a probability function and that there is at least one non null reward). 
    returns: true if the MDP is valid, false otherwise
     */
     pub fn validate(&self) -> bool{ 
        let mut sum_transition;
        let mut reward = 0.0;
        for s1 in &self.states{
            for a in &self.actions {
                sum_transition = 0.0; // resets the transition for this state
                for s2 in &self.states {
                    sum_transition = sum_transition + self.transitions.get_transition(s1, a, s2);
                    let current_reward = self.rewards.get_reward(s1, a, s2);
                    reward = reward + current_reward;
                }
                if !float_cmp::approx_eq!(f64, sum_transition, 1.0, ulps = 4) {
                    // if the values don't sum to 1 for this s1 and a
                    return false;
                }
            }
        }
        // if there is not at least one non-null reward
        if float_cmp::approx_eq!(f64, reward, 0.0, ulps = 4) {
            return false;
        }

        return true;
    }
}


impl MDPAgent<usize, usize, MatrixTransition, MatrixReward>{

    pub fn new_matrix(nb_states: usize, nb_actions: usize, t: MatrixTransition, r: MatrixReward, discount: f64) -> MDPAgent<usize, usize, MatrixTransition, MatrixReward>  
    {
        // create a hashset from enumeration 
        let state_set: HashSet<usize> = (0..nb_states).collect();
        let action_set: HashSet<usize> = (0..nb_actions).collect();

        MDPAgent {
            states: state_set,
            actions: action_set,
            transitions: t,
            rewards: r,
            discount,
        }
    }
}
////////////////////////////////////////////////////////////////////////////
// Std traits implementation    
// See https://rust-lang.github.io/api-guidelines/interoperability.html                                          //
////////////////////////////////////////////////////////////////////////////
impl<S, A, T, R> Clone for MDPAgent<S, A, T, R>  where 
S: Eq + Hash,
A: Eq + Hash,
T: TransitionModel<S, A>,
R: RewardModel<S, A>{

    fn clone(&self) -> Self { todo!() }

}

////////////////////////////////////////////////////////////////////////////
// Unit Tests                                                             //
////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test{
    use crate::mdp::{reward_model::MatrixReward, transition_model::MatrixTransition};

    use super::*;

    //////////////////////////////////////////////////////////////////////////// Couple of structures needed for the tests
    struct StudentTransitionModel {

    }
    
    impl TransitionModel<String, String> for StudentTransitionModel {
        fn get_transition(&self, s1: &String, a: &String, s2: &String) -> f64 {
            match (s1.as_str(), a.as_str()) {
                ("class1", "study") => {
                    match s2.as_str() {
                        "class1" => return 0.2,
                        "class2" => return 0.8,
                        _ => return 0.0
                    }
                }
                ("class1", "distract") => {
                    match s2.as_str() {
                        "facebook" => return 1.0,
                        _ => return 0.0
                    }
                }
                ("class2", "study") => {
                    match s2.as_str() {
                        "passed" => return 0.9,
                        "class2" => return 0.1,
                        _ => return 0.0
                    }
                }
                ("class2", "distract") => {
                    match s2.as_str() {
                        "facebook" => return 1.0, 
                        _ => return 0.0,
                    }
                }
                ("passed", _) => {
                    match s2.as_str() {
                        "passed" => return 1.0,
                        _ => return 0.0,
                    }
                }
                ("facebook", "study") => {
                    match s2.as_str() {
                        "class1" => return 0.5, 
                        "facebook" => return 0.5, 
                        _ => return 0.0
                    }
                }
                ("facebook", "distract") => {
                    match s2.as_str() {
                        "facebook" => 1.0,
                        _ => 0.0
                    }
                } 
                _ => 0.0
            }
        }

    }

    struct StudentRewardModel {}
    impl RewardModel<String, String> for StudentRewardModel {
        fn get_reward(&self, _s1: &String, _a: &String, s2: &String) -> f64 {
            if s2.as_str().eq("passed") {
                10.0
            }
            else {
                0.0
            }
        }

    }


    struct InvalidTransitionModel {

    }

    impl TransitionModel<String, String> for InvalidTransitionModel {
            fn get_transition(&self, _s1: &String,_a: &String, _s2: &String) -> f64 {
            return 0.5;
        }
    }

    struct InvalidRewardModel { }
    impl RewardModel<String, String> for InvalidRewardModel {
        fn get_reward(&self, _s1: &String, _a: &String, _s2: &String) -> f64 {
        return 0.0;
    }

    }


    //////////////////////////////////////////////////////////////////////////// Test Functions
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

        let mdp_agent = MDPAgent::new_matrix(3, 2, t, r, 0.9);

        assert!(mdp_agent.validate());
        }

    #[test]
    #[should_panic]
    fn create_invalid_transition_matrix_mdp() {
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

    #[test]
    #[should_panic]
    fn create_invalid_reward_matrix_mdp() {
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
                vec![vec![0.0, 0.0, 0.0], //a1
                     vec![0.0, 0.0, 0.0]],
                // s2
                vec![vec![0.0, 0.0, 0.0],
                     vec![0.0, 0.0, 0.0]],
                //s3
                vec![vec![0.0, 0.0, 0.0],
                     vec![0.0, 0.0, 0.0]]
            ];

        let r: MatrixReward = MatrixReward::new(reward_array);

        let mdp_agent = MDPAgent::new_matrix(3, 2, t, r, 0.9);

        assert!(mdp_agent.validate());
        }

    
    #[test]
    fn create_valid_mdp() {
        let states: HashSet<String> = ["class1", "class2", "facebook", "passed"].iter().map(|x| x.to_string()).collect();
        let actions: HashSet<String> = ["study", "distract"].iter().map(|x| x.to_string()).collect();
        let transition = StudentTransitionModel{

        };
        let reward = StudentRewardModel {

        };
        let agent = MDPAgent::new(states, actions, transition, reward, 0.9);
        assert!(agent.validate());
    }


    #[test]
    #[should_panic]
    fn create_invalid_transition_mdp() {
        let states: HashSet<String> = ["class1", "class2", "facebook", "passed"].iter().map(|x| x.to_string()).collect();
        let actions: HashSet<String> = ["study", "distract"].iter().map(|x| x.to_string()).collect();
        let transition = InvalidTransitionModel{

        };
        let reward = StudentRewardModel {

        };
        let agent = MDPAgent::new(states, actions, transition, reward, 0.9);
        assert!(agent.validate());
    }

        
    #[test]
    #[should_panic]
    fn create_invalid_reward_mdp() {
        let states: HashSet<String> = ["class1", "class2", "facebook", "passed"].iter().map(|x| x.to_string()).collect();
        let actions: HashSet<String> = ["study", "distract"].iter().map(|x| x.to_string()).collect();
        let transition = StudentTransitionModel{

        };
        let reward = InvalidRewardModel{

        };
        let agent = MDPAgent::new(states, actions, transition, reward, 0.9);
        assert!(agent.validate());
    }



}
