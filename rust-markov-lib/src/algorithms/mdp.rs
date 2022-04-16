use crate::models::{mdp_agent::MDPAgent, reward_model::RewardModel, transition_model::TransitionModel};
use std::collections::HashMap;
use std::hash::Hash;
use std::option::Option;


// ////////////////////////////////////////////////////////////////////////////
// // Value Iteration                                                        //
// ////////////////////////////////////////////////////////////////////////////
pub struct ValueIteration<'a, S, A, T, R> where 
S: Clone + Eq + Hash, 
A: Eq + Hash + Clone,
T: TransitionModel<S, A>,
R: RewardModel<S, A> {
    // parameters
    epsilon: f64, 
    mdp: &'a MDPAgent<S, A, T, R>,

    // Internal variables
    value_vector:  Option<HashMap<&'a S, f64>>, //contains the value of each state
    policy: Option<HashMap<&'a S, &'a A>> //contains the policy action for each state
}

impl<'a, S, A, T, R> ValueIteration<'a, S, A, T, R> where 
S: Clone + Eq + Hash, 
A: Eq + Hash + Clone,
T: TransitionModel<S, A>,
R: RewardModel<S, A> {
    pub fn new(mdp: &'a MDPAgent<S, A, T, R>, epsilon: f64) -> Self {
        ValueIteration {
            epsilon,
            mdp, 
            value_vector: None, 
            policy: None
        }
    }

    pub fn solve(&mut self) {
        // initialize the value_vector
        let mut utility_vector: HashMap<&S, f64> = self.mdp.states.iter().map(|x| (x, 0.0)).collect();
        let mut policy_vector: HashMap<&S, &A> = HashMap::new();
        // Previous utility vector
        let mut previous_utility_vector: HashMap<&S, f64>;

        // maximum relative change in utility of any state
        let mut delta;
    

        loop {
            delta = 0.0;
            previous_utility_vector = utility_vector.clone();
            
            for s in &self.mdp.states {
                // update the q-value function
                let mut max_utility = None;
                let mut best_action = None;
                // for each possible action
                for a in &self.mdp.actions {
                    let utility = self.q_value(s, a, &previous_utility_vector);
                    
                    match max_utility {
                    Some(mu) => {
                        if utility > mu {
                        max_utility = Some(utility);
                        best_action = Some(a);
                        }
                    },
                    None => {
                        max_utility = Some(utility);
                        best_action = Some(a);
                    },
                    };
                }
                // utility_vector was initialized to 0.0 so we know all the keys already exist
                utility_vector.insert(s, max_utility.unwrap()); // here S and A implement copy, so the values are copied in the hashmap

                //policy vector was not initialized fist, so we need to check if the key exists already
                *policy_vector.entry(s).or_insert(best_action.unwrap()) = best_action.unwrap(); 
                

                // we take the highest change in utility
                let new_delta = (utility_vector.get(&s).unwrap() - previous_utility_vector.get(&s).unwrap()).abs();
                if new_delta > delta {
                    delta = new_delta
                }
        
            }
            // stopping condition
            if delta <= (self.epsilon * (1.0 - self.mdp.discount) / self.mdp.discount) { break; }
        }
        self.value_vector = Some(utility_vector);
        self.policy = Some(policy_vector);
    }

    pub fn get_value_vector(&self) -> Option<&HashMap<&S, f64>> {
        match &self.value_vector {
            None => Option::None,
            Some(v) => Some(v),
        }
    }

    pub fn get_policy(&self) -> Option<&HashMap<&S, &A>> {
        match &self.policy {
            Some(p) => Some(p),
            None => None,
        }
    }

    pub fn q_value(&self, s: &S, a: &A, utility_vector: &HashMap<&S, f64>) -> f64 {
        let mut sum = 0.0;
        for s2 in &self.mdp.states {
            sum = sum + self.mdp.transitions.get_transition(s, a, s2) * 
                (self.mdp.rewards.get_reward(s, a, s2) + self.mdp.discount * (*utility_vector).get(s2).unwrap());
        }

        return sum;  
    }
}



////////////////////////////////////////////////////////////////////////////
// Tests                                                                  //
////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test{
    use std::collections::HashSet;

    use float_cmp::*;
    use super::*;

     //////////////////////////////////////////////////////////////////////////// Couple of structures needed for the tests
     struct StudentTransitionModel {}
    
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
        fn get_reward(&self, s1: &String, a: &String, s2: &String) -> f64 {
            if s1.as_str().eq("passed") && a.as_str().eq("distract") {
                2.0
            }
            else if s1.as_str().ne("passed") && s2.as_str().eq("passed"){
                10.0
            }
            else {
                0.0
            }
        }
    }

    fn create_student_mdp() -> MDPAgent<String, String, StudentTransitionModel, StudentRewardModel> {
        let states: HashSet<String> = ["class1", "class2", "facebook", "passed"].iter().map(|x| x.to_string()).collect();
        let actions: HashSet<String> = ["study", "distract"].iter().map(|x| x.to_string()).collect();
        let transitions= StudentTransitionModel{};
        let rewards = StudentRewardModel{};
        MDPAgent::new(states, actions, transitions, rewards, 0.9)
    }

    ////////////////////////////////////////////////////////////////////////// Actual test cases
    #[test]
    fn value_iteration_solve() {
        let mdp = create_student_mdp();
        let mut solver = ValueIteration::new(&mdp, 0.01);

        solver.solve();
        let optimal_utility_vector = solver.get_value_vector().unwrap();
        let optimal_policy = solver.get_policy().unwrap();
        //println!("Value Function");
        //optimal_utility_vector.iter().for_each(|(x, y)| println!("{} - {}", x, y));
        //println!("Policy");
        //optimal_policy.iter().for_each(|(x, y)| println!("{} - {}", x, y));
        for (s, v) in optimal_utility_vector {
            match s.as_str() {
                "class1" => assert!(approx_eq!(f64, *v, 24.3060, epsilon=0.0002),  "v = {}", *v),
                "class2" => assert!(approx_eq!(f64, *v, 27.6831, epsilon=0.0002),  "v = {}", *v),
                "facebook" => assert!(approx_eq!(f64, *v, 19.8851, epsilon=0.0002), "v = {}", *v),
                "passed" => assert!(approx_eq!(f64, *v, 19.9908, epsilon=0.0002),  "v = {}", *v),
                _ => panic!("State {} should not be present", s),
            }
        }

        for (s, a) in optimal_policy {
            match s.as_str() {
                "class1" => assert!(a.as_str().eq("study")),
                "class2" => assert!(a.as_str().eq("study")),
                "facebook" => assert!(a.as_str().eq("study")),
                "passed" => assert!(a.as_str().eq("distract")),
                _ => panic!("State {} should not be present", s),
            }
        }

    }
}

