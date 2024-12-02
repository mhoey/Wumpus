use std::ops::Range;
// Trait for abstracting random methods
// that can run on either an OS runtime 
// link to javascript random when running
// on wasm
pub trait Random {
    fn new() -> Self;
    fn get_random(&self, range: Range<u8>) -> u8;
    fn shuffle(&self, vec: Vec<u8>) -> Vec<u8>;
}