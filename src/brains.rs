use crate::cell::{CellBrain, Cell, ProceedResult};





#[derive(Default)]
pub struct DefaultBrain {

}

impl CellBrain for DefaultBrain {
    fn proceed(self: &Self, cell: &mut Cell) -> ProceedResult {
        todo!()
    }
}