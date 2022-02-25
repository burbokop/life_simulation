use core::num::dec2flt::number::Number;
use std::ops::Add;

use matmath::game::vec2::Vector2;
use num_traits::Signed;

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
 
    pub fn into_predicate<T: Signed + PartialOrd<i32>>(self: Neighborhood) -> fn(Vector2<T>) -> bool {
        match self {
            Neighborhood::Moore => |v: Vector2<T>| v.x.abs() <= 1 && v.y.abs() <= 1,
            Neighborhood::VonNeumann => |v: Vector2<T>| (v.x.abs() <= 1 && v.y == 0) || (v.y.abs() <= 1 && v.x == 0)            
        }
    }

    pub fn predicate<T: Signed + PartialOrd<i32>>(self: &Neighborhood) -> fn(&Vector2<T>) -> bool {
        match self {
            Neighborhood::Moore => |v| v.x.abs() <= 1 && v.y.abs() <= 1,
            Neighborhood::VonNeumann => |v| (v.x.abs() <= 1 && v.y == 0) || (v.y.abs() <= 1 && v.x == 0)            
        }
    }

    //iter: impl Iterator<Item = PositionalCell>
    pub fn into_filter_fn(self: Neighborhood, c0: PositionalCell) -> impl FnOnce(PositionalCell) -> bool {
        |c1| self.into_predicate()(c0.position - c1.position)
    }
    //pub fn filter_fn(self: &Neighborhood, c0: &PositionalCell) -> impl Fn(&PositionalCell) -> bool {
    //    |c1| self.predicate()(&(c0.position.clone() - c1.position.clone()))
    //}

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

                if result.ready_mate {
                    self
                        .cells
                        .iter()
                        .filter(|c| Neighborhood::Moore.into_predicate()(self.cells[i].position - c.position))
                        .filter(|c| c.)

                }
            }
        }

        for i in dead_indices.into_iter() {
            self.cells.remove(i);
        }
    }
}