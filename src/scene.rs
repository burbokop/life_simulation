use crate::cell::{Cell, CellBrain};

#[derive(Default)]
pub struct Scene {
    pub cells: Vec<Cell>
}

impl Scene {
    pub fn proceed<B: CellBrain>(self: &mut Self, brain: &B) -> () {
        self.cells.iter_mut().for_each(|cell| {
            cell.proceed(brain);
        });
    }
}