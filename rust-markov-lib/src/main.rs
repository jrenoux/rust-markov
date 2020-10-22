
//mod markov_chain;
extern crate vose_alias;

// use markov_chain::model::MarkovChain;
use std::collections::HashMap;

fn main() {
   
    // let states = 3;
    // let transitions = vec![vec![0.1,0.8,0.1],vec![0.3, 0.2, 0.5],vec![0.5, 0.3, 0.2]];

    // let mc = MarkovChain::new(states, transitions);

    // println!("{}", mc.to_string());

    let va = vose_alias::VoseAlias::new(vec![1, 2, 3, 4], vec![0.7, 0.1, 0.1, 0.1]);
   

    let mut nb_drawn:HashMap<u16, f32> = HashMap::new();
    let nb_samples = 100000;
    for _i in 1..=nb_samples {
	if let Some(t) = va.sample() {
	    nb_drawn.entry(t).or_insert(1.0);
	    if let Some(nb) = nb_drawn.get(&t){
		let incremented_nb = nb + 1.0;
		nb_drawn.insert(t, incremented_nb);
	    };
	}
    }

    for key in nb_drawn.keys() {
	if let Some(nb) = nb_drawn.get(&key) {
	    let stat = nb / nb_samples as f32;
	    println!("{} -> {}", key, stat);
	}
    }


}
