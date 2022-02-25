


use matmath::game::vec2::Vector2;
use chromosome::Chromosome;
use rand::{RngCore, distributions::uniform::SampleRange, Rng};

pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl<T: From<i8>> From<Direction> for Vector2<T> {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Left => Vector2::new(T::from(-1_i8), T::from(0_i8)),
            Direction::Right => Vector2::new(T::from(1_i8), T::from(0_i8)),
            Direction::Up => Vector2::new(T::from(0_i8), T::from(-1_i8)),
            Direction::Down => Vector2::new(T::from(0_i8), T::from(1_i8)),
        }
    }
}

pub struct ProceedResult {
    pub move_direction: Direction,
    pub is_dead: bool,
    pub ready_mate: bool
}

pub trait CellBrain {
    fn proceed(self: &Self, cell: &mut Cell) -> ProceedResult;
}


pub struct Cell {
    chromosome: Chromosome<u8>,
    current_cmd_index: usize,
}

impl Cell {
    pub fn new_random<R : RngCore, Range: SampleRange<u8> + Clone>(size: usize, range: Range, rng: &mut R) -> Self {
        Cell { chromosome: Chromosome::new_random(size, range, rng), current_cmd_index: 0 }
    }

    pub fn proceed<T: CellBrain>(self: &mut Self, brain: &T) -> ProceedResult {
        brain.proceed(self)
    }
}