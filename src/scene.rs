use std::ops::Add;

use matmath::game::vec2::Vector2;

use crate::cell::{Cell, CellBrain};



pub struct PositionalCell {
    pub cell: Cell,
    pub position: Vector2<i32>
}


#[derive(Default)]
pub struct Scene {
    pub cells: Vec<PositionalCell>
}


impl Scene {
    pub fn proceed<B: CellBrain>(self: &mut Self, brain: &B) -> () {
        let mut dead_indices: Vec<usize> = Vec::with_capacity(self.cells.len());
        for i in 0..self.cells.len() {
            let result = self.cells[i].cell.proceed(brain);
            if result.is_dead {
                dead_indices.push(i)
            } else {
                let vec: Vector2<i32> = self.cells[i].position.add(result.move_direction.into());
                

                for j in 0..self.cells.len() {


                }
            }
        }

        for i in dead_indices.into_iter() {
            self.cells.remove(i);
        }
    }
}