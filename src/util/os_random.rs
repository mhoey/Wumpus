use std::ops::Range;
use rand::prelude::*;
use rand::seq::SliceRandom;
use super::random::Random;

#[derive(Clone, Copy)] 
pub struct OsRandom;
impl Random for OsRandom {
    fn new() -> Self {
        OsRandom
    }

    fn get_random(&self, range: Range<u8>) -> u8 {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(range);
        random_number
    }

    fn shuffle(&self, vector: Vec<u8>) -> Vec<u8> {    
        let mut vector_to_shuffle = vector.to_vec();
        let mut rng = rand::thread_rng();
        vector_to_shuffle.shuffle(&mut rng);
        vector_to_shuffle
    }
}