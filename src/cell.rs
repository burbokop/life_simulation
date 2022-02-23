

use std::slice::SliceIndex;

use al_jabr::{Vector2, vector, Matrix};
use chromosome::Chromosome;
use rand::{RngCore, distributions::uniform::SampleRange, Rng};

enum Direction {
    Left,
    Right,
    Up,
    Down
}


impl<T: From<i8>> From<Direction> for Vector2<T> {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Left => vector![T::from(-1_i8), T::from(0_i8)],
            Direction::Right => vector![T::from(1_i8), T::from(0_i8)],
            Direction::Up => vector![T::from(0_i8), T::from(-1_i8)],
            Direction::Down => vector![T::from(0_i8), T::from(1_i8)],
        }
    }
}


pub trait CellBrain {
    fn proceed(self: &Self, chromosome: &Chromosome<u8>, current_cmd_index: &mut usize) -> bool;

    fn move_cell() -> 
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