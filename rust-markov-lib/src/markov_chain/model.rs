
use float_cmp::*;
extern crate vose_alias;

use vose_alias::VoseAlias;


////////////////////////////////////////////////////////////////////////////
// Structure Definition                                                   //
////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct MarkovChain{
    pub states:usize,
    pub transitions:Vec<Vec<f32>>,
    pub vose_alias_tables:Vec<VoseAlias<usize>>,
    _private: () // this is to prevent the creation of the structure without using the new() method while keeping the states and transitions accessibles
    
}


////////////////////////////////////////////////////////////////////////////
// MarkovChain Structure Implementation                                   //
////////////////////////////////////////////////////////////////////////////

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
        }

	let mut element_vector:Vec<usize> = Vec::new();
	// Preparing the list of states from the number of states
	
	for s in 0..states {
	   element_vector.push(s);
	}
	
	// creates the VA table for each state
	let mut va_tables:Vec<VoseAlias<usize>> = Vec::new();
	for s in 0 .. states {
	    match transitions.get(s) {
		Some(t) => {
		    let va = VoseAlias::new(element_vector.clone(), t.clone());
		    println!("{}", va);
		    va_tables.push(va);

		},
		None => {
		    panic!("Internal error, could not find transition for state {}", s);
		}
	    }

	    
	}
	

        let mc = MarkovChain {
            states: 3,
            transitions: transitions,
	    vose_alias_tables: va_tables,
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

        format!("s: {},\nt: \n{}\n", self.states, t)
    }

    pub fn random_walk(&self, starting_state: usize, walk_length:usize) -> Vec<usize> {
        let mut walk_sequence:Vec<usize> = Vec::new();
        let mut current_state = starting_state;
        walk_sequence.push(starting_state);

        for _i in 0..walk_length {
            //compute a new state according to transitions
	    match self.vose_alias_tables.get(current_state) {
		Some(va) => {
		    // we got the table corresponding to the state, now we sample
		    let new_s = va.sample();
		    walk_sequence.push(new_s);
		    current_state = new_s;
		    }
		    
		
		None => {
		    panic!("Internal error. Impossible to get the Vose-Alias table for current state {}", current_state);
		}
	    }
	    
            //
        }


        return walk_sequence;
    }
}


////////////////////////////////////////////////////////////////////////////
// Unit Tests                                                             //
////////////////////////////////////////////////////////////////////////////
// TODO
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creation_ok() {
	let states = 3;
	let transitions = vec![vec![0.0, 0.5, 0.5],vec![0.0, 0.8, 0.2],vec![0.8, 0.0, 0.2]];

	let mc = MarkovChain::new(states, transitions);

	println!("{}", mc.to_string());
    
	for s in 0 .. mc.states {
	    println!("{:?}", mc.vose_alias_tables.get(s));
	}

	// perform a random walk
	let random_walk = mc.random_walk(0, 30);
	for s in random_walk{
	    print!("{} - ", s);
	}
    }

    #[test]
    fn creation_not_ok() {
    }

    #[test]
    fn display_ok() {
    }

    #[test]
    fn random_walk() {
    }
}
