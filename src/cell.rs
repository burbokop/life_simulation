

use std::slice::SliceIndex;

use chromosome::Chromosome;
use rand::{RngCore, distributions::uniform::SampleRange, Rng};


pub trait CellBrain {
    fn proceed(self: &Self, chromosome: &Chromosome<u8>, current_cmd_index: &mut usize) -> bool;
}


pub struct Cell {
    chromosome: Chromosome<u8>,
    current_cmd_index: usize,
    pub x: i32,
    pub y: i32
}

impl Cell {
    pub fn new_random<R : RngCore, Range: SampleRange<u8> + Clone>(size: usize, range: Range, rng: &mut R, x: i32, y: i32) -> Self {
        Cell { chromosome: Chromosome::new_random(size, range, rng), current_cmd_index: 0, x: x, y: y }
    }

    pub fn proceed<T: CellBrain>(self: &mut Self, brain: &T) -> bool {
        brain.proceed(&self.chromosome, &mut self.current_cmd_index)
    }
}