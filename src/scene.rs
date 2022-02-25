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

enum Neighborhood {
    Moore,
    VonNeumann,
}

impl Neighborhood {
    pub fn max_count(self: Neighborhood) -> usize {
        match self {
            Neighborhood::Moore => 8,
            Neighborhood::VonNeumann => 4,
        }
    }

    //iter: impl Iterator<Item = PositionalCell>
    pub fn filter_fn(self: Neighborhood) -> fn(&PositionalCell, &PositionalCell) -> bool {
        match self {
            Neighborhood::Moore => {
                |center: &PositionalCell, c: &PositionalCell| -> bool {(c.position.x - center.position.x).abs() <= 1 && (c.position.y - center.position.y).abs() <= 1}
            },
            Neighborhood::VonNeumann => {
                |center: &PositionalCell, c: &PositionalCell| -> bool {(c.position.x - center.position.x).abs() <= 1 || (c.position.y - center.position.y).abs() <= 1}
            },
        }
    }
}

impl Scene {


    pub fn proceed<B: CellBrain>(self: &mut Self, brain: &B) -> () {
        let mut dead_indices: Vec<usize> = Vec::with_capacity(self.cells.len());
        for i in 0..self.cells.len() {
            let result = self.cells[i].cell.proceed(brain);
            if result.is_dead {
                dead_indices.push(i)
            } else {
                let direction: Vector2<i32> = result.move_direction.into();            
                let new_position=  self.cells[i].position.clone() + direction;
                if self.cells.iter().find(|c| new_position == c.position).is_none() {
                    self.cells[i].position = new_position;
                }

                if result.ready_mate &&  {

                }
            }
        }

        for i in dead_indices.into_iter() {
            self.cells.remove(i);
        }
    }
}