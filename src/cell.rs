

use std::slice::SliceIndex;

use chromosome::Chromosome;
use rand::{RngCore, distributions::uniform::SampleRange, Rng};


trait CellBrain {
    fn proceed(chromosome: &Chromosome<u8>, current_cmd_index: &mut usize) -> bool;
}


pub struct Cell {
    chromosome: Chromosome<u8>,
    current_cmd_index: usize
}

impl Cell {
    pub fn new_random<R : RngCore, Range: SampleRange<u8> + Clone>(size: usize, range: Range, rng: &mut R) -> Self {
        Cell { chromosome: Chromosome::new_random(size, range, rng), current_cmd_index: 0 }
    }

    pub fn proceed<T: CellBrain>(self: &Self, brain: &T) -> bool {
        brain.proceed(self.chromosome, self.current_cmd_index)
    }
}