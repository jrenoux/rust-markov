use std::fmt::Display;
use std::hash::Hash;
use std::{collections::{HashSet}};
use crate::models::{emission_model::EmissionModel, reward_model::RewardModel, transition_model::TransitionModel};

use super::emission_model::MatrixEmission;
use super::reward_model::MatrixReward;
use super::transition_model::MatrixTransition;
pub struct POMDPAgent<S, A, O, T, E, R> where
    S: Eq + Hash + Clone + Display,
    A: Eq + Hash + Clone + Display,
    O: Eq + Hash + Clone + Display,
    T: TransitionModel<S, A>,
    E: EmissionModel<S, A, O>,
    R: RewardModel<S, A> {
        pub states: HashSet<S>,
        pub actions : HashSet<A> ,
        pub observations: HashSet<O>,
        pub transitions: T,
        pub emissions : E, 
        pub rewards: R,
        pub discount: f64
    }

impl<S, A, O, T, E, R> POMDPAgent<S, A, O, T, E, R> where 
S: Eq + Hash + Clone + Display,
A: Eq + Hash + Clone + Display,
O: Eq + Hash + Clone + Display,
T: TransitionModel<S, A>,
E: EmissionModel<S, A, O>,
R: RewardModel<S, A> {
    pub fn new(states: HashSet<S>, actions: HashSet<A>, observations: HashSet<O>, transitions: T, emissions: E, rewards: R, discount: f64) -> Self {
        POMDPAgent{
            states,
            actions,
            observations,
            transitions,
            emissions, 
            rewards, 
            discount,
        }
    }

    pub fn validate(&self) -> bool {
        self.validate_transition_reward() && self.validate_emission()
    }

    fn validate_transition_reward(&self) -> bool {
        let mut sum_transition;
        let mut reward = 0.0;
        for s1 in &self.states{
            for a in &self.actions {
                sum_transition = 0.0; // resets the transition for this state
                for s2 in &self.states {
                    sum_transition = sum_transition + self.transitions.get_transition(s1, a, s2);
                    let current_reward = self.rewards.get_reward(s1, a, s2);
                    reward = reward + current_reward.abs();
                }
                if !float_cmp::approx_eq!(f64, sum_transition, 1.0, epsilon=0.01) {
                    // if the values don't sum to 1 for this s1 and a
                    return false;
                }
            }
        }
        // if there is not at least one non-null reward
        if float_cmp::approx_eq!(f64, reward, 0.0, epsilon=0.01) {
            return false;
        }
        return true;
    }

    fn validate_emission(&self) -> bool{
        let mut sum_emission;
        for s1 in &self.states{
            for a in &self.actions {
                sum_emission = 0.0; // resets the transition for this state
                for o in &self.observations {
                    sum_emission = sum_emission + self.emissions.get_emission(s1, a, o);
                }
                if !float_cmp::approx_eq!(f64, sum_emission, 1.0, ulps = 4) {
                    // if the values don't sum to 1 for this s1 and a
                    return false;
                }
            }
        }
        return true;

    }
}

impl POMDPAgent<usize, usize, usize, MatrixTransition, MatrixEmission, MatrixReward> {
    pub fn new_matrix(  nb_states: usize, 
                        nb_actions: usize, 
                        nb_observation: usize, 
                        t: MatrixTransition, 
                        e: MatrixEmission,
                        r: MatrixReward, 
                        discount: f64) 
            -> POMDPAgent<usize, usize, usize, MatrixTransition, MatrixEmission, MatrixReward>  {
        // create a hashset from enumeration 
        let state_set: HashSet<usize> = (0..nb_states).collect();
        let action_set: HashSet<usize> = (0..nb_actions).collect();
        let observation_set: HashSet<usize> = (0..nb_observation).collect();

        POMDPAgent {
            states: state_set,
            actions: action_set,
            observations: observation_set,
            transitions: t,
            emissions: e,
            rewards: r,
            discount,
        }
    }
}

////////////////////////////////////////////////////////////////////////////
// Std traits implementation    
// See https://rust-lang.github.io/api-guidelines/interoperability.html                                          //
////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////
// Unit Tests                                                             //
////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
    use super::*;

    struct TigerTransitionModel {}
    impl TransitionModel<String, String> for TigerTransitionModel {
        fn get_transition(&self, s1: &String, a: &String, s2: &String) -> f64 {
            if s1.eq(s2) {
                return 1.0;
            }
            return 0.0;
        }
    }

    struct InvalidTigerTransitionModel{}
    impl TransitionModel<String, String> for InvalidTigerTransitionModel {
        fn get_transition(&self, s1: &String, a: &String, s2: &String) -> f64 {
            return 1.0;
        }
    }

    struct TigerEmissionModel{}
    impl EmissionModel<String, String, String> for TigerEmissionModel {
        fn get_emission(&self, s: &String, a: &String, o: &String) -> f64 {
            match (s.as_str(), a.as_str()) {
                ("tiger_left", "listen") => match o.as_str() {
                    "roar_left" => 0.8,
                    "roar_right" => 0.2, 
                    "nothing" => 0.0,
                    _ => panic!("Should not be here")
                }
                ("tiger_right", "listen") => match o.as_str() {
                    "roar_right" => 0.8,
                    "roar_left" => 0.2,
                    "nothing" => 0.0,
                    _ => panic!("Should not be here")   
                }
                (_, "open_left") => match o.as_str() {
                    "nothing" => 1.0,
                    _ => 0.0,
                }
                (_, "open_right") => match o.as_str() {
                    "nothing" => 1.0, 
                    _ => 0.0,
                }
                _ => panic!("Should not be here"),
            }
        }
    }

    struct InvalidTigerEmissionModel {}
    impl EmissionModel<String, String, String> for InvalidTigerEmissionModel {
        fn get_emission(&self, s: &String, a: &String, o: &String) -> f64 {
            return 0.9;
        }
    }

    struct TigerRewardModel {}
    impl RewardModel<String, String> for TigerRewardModel {
        fn get_reward(&self, s1: &String, a: &String, s2: &String) -> f64{
            match (s1.as_str(), a.as_str()) {
                ("tiger_left", "open_left") => -100.0,
                ("tiger_left", "open_right") => 100.0,
                ("tiger_right", "open_left") => 100.0,
                ("tiger_right", "open_right") => -100.0,
                (_, "listen") => 0.0,
                _ => {panic!("Should not reach here..."); 0.0}
            }
        }
    }

    struct InvalidTigerRewardModel {}
    impl RewardModel<String, String> for InvalidTigerRewardModel {
        fn get_reward(&self, s1: &String, a: &String, s2: &String) -> f64 {
            return 0.0;
        }
    }

    //////////////////////////////////////////////////////////////////////////// Test Functions
    #[test]
    fn create_valid_pomdp() {
        let states: HashSet<String> = ["tiger_left", "tiger_right"].iter().map(|x| x.to_string()).collect();
        let actions: HashSet<String> = ["open_left", "open_right", "listen"].iter().map(|x| x.to_string()).collect();
        let observations: HashSet<String> = ["roar_left", "roar_right", "nothing"].iter().map(|x| x.to_string()).collect();
        let transition = TigerTransitionModel{};
        let emission = TigerEmissionModel{};
        let reward = TigerRewardModel{};
        let agent = POMDPAgent::new(states, 
                                                                     actions, 
                                                                     observations, 
                                                                     transition, 
                                                                     emission, 
                                                                     reward, 
                                                                     0.9);
        assert!(agent.validate());

    }


    #[test]
    #[should_panic]
    fn create_invalid_transition_pomdp() {
        let states: HashSet<String> = ["tiger_left", "tiger_right"].iter().map(|x| x.to_string()).collect();
        let actions: HashSet<String> = ["open_left", "open_right", "listen"].iter().map(|x| x.to_string()).collect();
        let observations: HashSet<String> = ["roar_left", "roar_right", "nothing"].iter().map(|x| x.to_string()).collect();
        let transition = InvalidTigerTransitionModel{};
        let emission = TigerEmissionModel{};
        let reward = TigerRewardModel{};
        let agent = POMDPAgent::new(states, 
                                                                        actions, 
                                                                        observations, 
                                                                        transition, 
                                                                        emission, 
                                                                        reward, 
                                                                        0.9);
        assert!(agent.validate());
    }

    #[test]
    #[should_panic]
    fn create_invalid_reward_pomdp() {
        let states: HashSet<String> = ["tiger_left", "tiger_right"].iter().map(|x| x.to_string()).collect();
        let actions: HashSet<String> = ["open_left", "open_right", "listen"].iter().map(|x| x.to_string()).collect();
        let observations: HashSet<String> = ["roar_left", "roar_right", "nothing"].iter().map(|x| x.to_string()).collect();
        let transition = TigerTransitionModel{};
        let emission = TigerEmissionModel{};
        let reward = InvalidTigerRewardModel{};
        let agent = POMDPAgent::new(states, 
                                                                        actions, 
                                                                        observations, 
                                                                        transition, 
                                                                        emission, 
                                                                        reward, 
                                                                        0.9);
        assert!(agent.validate());

    }

    #[test]
    #[should_panic]
    fn create_invalid_emission_pomdp() {
        let states: HashSet<String> = ["tiger_left", "tiger_right"].iter().map(|x| x.to_string()).collect();
        let actions: HashSet<String> = ["open_left", "open_right", "listen"].iter().map(|x| x.to_string()).collect();
        let observations: HashSet<String> = ["roar_left", "roar_right", "nothing"].iter().map(|x| x.to_string()).collect();
        let transition = TigerTransitionModel{};
        let emission = InvalidTigerEmissionModel{};
        let reward = TigerRewardModel{};
        let agent = POMDPAgent::new(states, 
                                                                        actions, 
                                                                        observations, 
                                                                        transition, 
                                                                        emission, 
                                                                        reward, 
                                                                        0.9);
        assert!(agent.validate());
    }

    #[test]
    fn create_valid_matrix_pomdp() {
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

        let emission_array = vec![
            vec![vec![0.8, 0.2],
                 vec![0.5, 0.5]],
            vec![vec![1.0, 0.0],
                 vec![0.9, 0.1]],
            vec![vec![0.0, 1.0],
                 vec![0.0, 1.0]]
        ];

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

        let t: MatrixTransition = MatrixTransition::new(transition_array);
        let e: MatrixEmission = MatrixEmission::new(emission_array);
        let r: MatrixReward = MatrixReward::new(reward_array);
        let pomdp_agent = POMDPAgent::new_matrix(3, 2, 2, t, e, r, 0.9);
        assert!(pomdp_agent.validate());
    }

    #[test]
    #[should_panic]
    fn create_invalid_transition_matrix_pomdp() {
        let transition_array = vec![
            //s1
            vec![vec![0.2, 0.8, 0.0], //a1
                 vec![0.3, 0.3, 0.4]],
            // s2
            vec![vec![0.8, 0.5, 0.1],
                 vec![0.1, 0.1, 0.8]],
            //s3
            vec![vec![0.2, 0.7, 0.1],
                 vec![0.5, 0.0, 0.5]]

        ];

        let emission_array = vec![
            vec![vec![0.8, 0.2],
                 vec![0.5, 0.5]],
            vec![vec![1.0, 0.0],
                 vec![0.9, 0.1]],
            vec![vec![0.0, 1.0],
                 vec![0.0, 1.0]]
        ];

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

        let t: MatrixTransition = MatrixTransition::new(transition_array);
        let e: MatrixEmission = MatrixEmission::new(emission_array);
        let r: MatrixReward = MatrixReward::new(reward_array);
        let pomdp_agent = POMDPAgent::new_matrix(3, 2, 2, t, e, r, 0.9);
        assert!(pomdp_agent.validate());
    }

    #[test]
    #[should_panic]
    fn create_invalid_emission_matrix_pomdp() {
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

        let emission_array = vec![
            vec![vec![0.8, 0.3],
                 vec![0.5, 0.5]],
            vec![vec![1.0, 0.0],
                 vec![0.9, 0.1]],
            vec![vec![0.0, 1.0],
                 vec![0.0, 1.0]]
        ];

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

        let t: MatrixTransition = MatrixTransition::new(transition_array);
        let e: MatrixEmission = MatrixEmission::new(emission_array);
        let r: MatrixReward = MatrixReward::new(reward_array);
        let pomdp_agent = POMDPAgent::new_matrix(3, 2, 2, t, e, r, 0.9);
        assert!(pomdp_agent.validate());
    }

    #[test]
    #[should_panic]
    fn create_invalid_reward_matrix_pomdp() {
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

        let emission_array = vec![
            vec![vec![0.8, 0.2],
                 vec![0.5, 0.5]],
            vec![vec![1.0, 0.0],
                 vec![0.9, 0.1]],
            vec![vec![0.0, 1.0],
                 vec![0.0, 1.0]]
        ];

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

        let t: MatrixTransition = MatrixTransition::new(transition_array);
        let e: MatrixEmission = MatrixEmission::new(emission_array);
        let r: MatrixReward = MatrixReward::new(reward_array);
        let pomdp_agent = POMDPAgent::new_matrix(3, 2, 2, t, e, r, 0.9);
        assert!(pomdp_agent.validate());
    }
    
}