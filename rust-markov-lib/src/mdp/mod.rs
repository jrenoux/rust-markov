pub mod simple_mdp;
pub mod algorithms;

pub trait MDP {
    fn get_nb_states(&self) -> usize;
    fn get_nb_actions(&self) -> usize;
    fn get_transition_probabilitiy(&self, s1:usize, a:usize, s2:usize) -> f32;
    fn get_reward(&self, s1:usize, a:usize, s2:usize) -> f32;
    fn get_discount_factor(&self) -> f32;
}



