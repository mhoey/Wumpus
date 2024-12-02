use std::ops::Range;

use super::random::Random;

type RandomFunction=fn(u8) -> u8;

fn empty_random(_: u8) -> u8 {
    return 0;
}

#[derive(Clone, Copy)] 
pub struct WasmRandom {
    random_function:RandomFunction
}

impl WasmRandom {
    pub fn set_random_function(&mut self, rf: RandomFunction) {
        self.random_function = rf;
    }
}

impl Random for WasmRandom {
    fn new() -> Self {
        WasmRandom { random_function: empty_random}
    }

    fn get_random(&self, range: Range<u8>) -> u8 {
        let r = (range.end + 1) - range.start;
        
        return (self.random_function)(r) + range.start;
    }

    fn shuffle(&self, vector: Vec<u8>) -> Vec<u8> {    
        let mut shuffled_vector = vector.to_vec();
        let mut i = vector.len()-1;
        while i > 0 {
            let ix = i as u8;
            let r = (self.random_function)(ix + 1);
            let temp = shuffled_vector[i];
            shuffled_vector[i] = vector[r as usize];
            shuffled_vector[r as usize] = temp;
            i = i - 1;
        }
        shuffled_vector
    }
}