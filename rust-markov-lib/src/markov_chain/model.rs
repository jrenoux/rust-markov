
use float_cmp::*;


pub struct MarkovChain{
    pub states:usize,
    pub transitions:Vec<Vec<f32>>,
    _private: () // this is to prevent the creation of the structure without using the new() method while keeping the states and transitions accessibles
    
}

impl MarkovChain {
    pub fn new(states:usize, transitions:Vec<Vec<f32>>) -> MarkovChain {
        // check that the transitions hav the right format
        let size_x:usize = transitions.len();
        if size_x != states {
            panic!("[Malformed Transition Function]  X_length = {} for {} states", size_x, states);
        }
        for i in &transitions {
            if i.len() != states {
                panic!("[Malformed Transition Function] Y_length = {} for {} states", i.len(), states);
            }
            let mut sum:f32 = 0.;
            for j in i {
                if j > &1.0 {
                    panic!("[Malformed Transition Function] {} !<= 1", j);
                }
                sum = sum + *j;
            }
            if  ! approx_eq!(f32, sum, 1.0, ulps=4) {
                panic!("[Malformed Transition Function] Sum {} != 1", sum);
            }
            println!("{}",sum);
        }

        let mc = MarkovChain {
            states: 3,
            transitions: transitions,
            _private: ()
        };

        return mc;
                    
    }
    
    pub fn to_string(&self) -> String {
        let mut t:String = String::from("[");
        let mut index = 0;
        for i in &self.transitions {
            for j in i {
                t = format!("{} {}",t, j);
                index = index + 1;
                if index == (self.states * self.states) {
                    t = t + " ]";                    
                }
                else if index%self.states == 0 {
                    t = format!("{}\n ", t);
                }
                
            }
        }

        format!("s: {},\nt: \n{}", self.states, t)
    }

    pub fn random_walk(&self, starting_state: u16, walk_length:u16) -> Vec<u16> {
        let mut walk_sequence:Vec<u16> = Vec::new();
        let mut current_state = starting_state;
        walk_sequence.push(starting_state);

        for i in 1..walk_length {
            //compute a new state according to transitions

            //
        }


        return walk_sequence;
    }
}
